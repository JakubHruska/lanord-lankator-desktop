use futures_util::StreamExt;
use log::{trace, debug, error, info, warn};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn download_package(
    url: String,
    download_path: String,
    slug: String,
) -> Result<String, String> {
    info!(
        "[download_package] Zahajuji stahování: {} -> {}/{}",
        url, download_path, slug
    );

    // 1. Vytvoření cílové složky
    let game_dir = Path::new(&download_path).join(&slug);
    fs::create_dir_all(&game_dir).map_err(|e| {
        let msg = format!(
            "Nepodařilo se vytvořit složku '{}': {}",
            game_dir.display(),
            e
        );
        error!("{}", msg);
        msg
    })?;

    // 2. Stažení souboru přes reqwest
    let response = reqwest::get(&url).await.map_err(|e| {
        let msg = format!("Chyba HTTP požadavku: {}", e);
        error!("{}", msg);
        msg
    })?;

    if !response.status().is_success() {
        let msg = format!(
            "Server vrátil chybu: {} {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("")
        );
        warn!("{}", msg);
        return Err(msg);
    }

    let content_length = response.content_length();
    info!(
        "[download_package] Velikost souboru: {}",
        content_length.map_or("neznámá".to_string(), |b| format!("{:.2} MB", b as f64 / 1_048_576.0))
    );

    // 3. Streamování přímo na disk po chunkách — bez načítání do RAM
    let zip_path = game_dir.join("temp.zip");
    let mut file = tokio::fs::File::create(&zip_path).await.map_err(|e| {
        let msg = format!("Nepodařilo se vytvořit soubor '{}': {}", zip_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| {
            let msg = format!("Chyba při čtení chunku: {}", e);
            error!("{}", msg);
            msg
        })?;
        file.write_all(&chunk).await.map_err(|e| {
            let msg = format!("Chyba při zápisu na disk: {}", e);
            error!("{}", msg);
            msg
        })?;
        downloaded += chunk.len() as u64;
        if let Some(total) = content_length {
            trace!(
                "[download_package] Průběh: {:.1} / {:.1} MB ({:.0}%)",
                downloaded as f64 / 1_048_576.0,
                total as f64 / 1_048_576.0,
                downloaded as f64 / total as f64 * 100.0
            );
        }
    }

    file.flush().await.map_err(|e| {
        let msg = format!("Chyba při flush souboru: {}", e);
        error!("{}", msg);
        msg
    })?;

    if downloaded == 0 {
        warn!("[download_package] Server vrátil prázdný soubor");
        return Err("Server vrátil prázdný soubor".to_string());
    }

    info!("[download_package] Staženo {:.2} MB -> {}", downloaded as f64 / 1_048_576.0, zip_path.display());
    Ok(zip_path.to_string_lossy().to_string())
}

#[derive(Deserialize)]
struct ManifestData {
    slug: String,
    #[serde(rename = "type")]
    package_type: String,
    title: String,
    description: String,
    file_size: u64,
    archive_file: String,
    created_at: Option<String>,
    #[serde(default = "default_status")]
    status: String,
}

fn default_status() -> String {
    "installing".to_string()
}

#[tauri::command]
async fn save_manifest(
    download_path: String,
    slug: String,
    manifest: serde_json::Value,
) -> Result<(), String> {
    // Log surového payloadu pro debug
    match serde_json::to_string_pretty(&manifest) {
        Ok(s) => debug!("[save_manifest] Přijatý manifest: {}", s),
        Err(e) => warn!("[save_manifest] Nelze serializovat přijatý manifest pro log: {}", e),
    }

    // Deserialize na typované data
    let manifest_data: ManifestData = serde_json::from_value(manifest).map_err(|e| {
        let msg = format!("Nepodařilo se deserialize manifestu: {}", e);
        error!("{}", msg);
        msg
    })?;

    // 1) Validace slug - jen alfanumerické + ._- povoleno, žádné / \\ nebo ".."
    if slug.trim().is_empty() {
        return Err("Slug je prázdný".to_string());
    }
    if slug.contains('/') || slug.contains('\\') || slug.contains("..") {
        return Err("Neplatný slug: obsahuje zakázané znaky".to_string());
    }
    for c in slug.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err("Neplatný slug: povoleny pouze A-Z, a-z, 0-9, '-', '_' a '.'".to_string());
        }
    }

    // 2) Zajisti, že základní cesta existuje a canonicalizuj ji
    fs::create_dir_all(&download_path).map_err(|e| {
        let msg = format!("Nepodařilo se vytvořit základní složku '{}': {}", download_path, e);
        error!("{}", msg);
        msg
    })?;
    let base_canon = fs::canonicalize(&download_path).map_err(|e| {
        let msg = format!("Nepodařilo se canonicalizovat '{}': {}", download_path, e);
        error!("{}", msg);
        msg
    })?;

    // 3) Složka pro hru musí být pod base_canon
    let game_dir = base_canon.join(&slug);
    if !game_dir.starts_with(&base_canon) {
        let msg = format!("Neplatná cílová složka po spojení: '{}'", game_dir.display());
        error!("{}", msg);
        return Err(msg);
    }

    // Vytvoř cílovou složku
    fs::create_dir_all(&game_dir).map_err(|e| {
        let msg = format!("Nepodařilo se vytvořit složku '{}': {}", game_dir.display(), e);
        error!("{}", msg);
        msg
    })?;

    // 4) Sanitize archive_file: použij jen basename (file_name)
    let archive_basename = std::path::Path::new(&manifest_data.archive_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("archive.zip")
        .to_string();

    let manifest_path = game_dir.join("manifest.json");
    info!(
        "[save_manifest] Ukládám manifest pro '{}' do '{}'",
        slug,
        manifest_path.display()
    );

    let json = serde_json::json!({
        "slug": manifest_data.slug,
        "type": manifest_data.package_type,
        "title": manifest_data.title,
        "description": manifest_data.description,
        "file_size": manifest_data.file_size,
        "archive_file": archive_basename,
        "created_at": manifest_data.created_at,
        "status": "installing",
    });

    let content = serde_json::to_string_pretty(&json).map_err(|e| {
        let msg = format!("Nepodařilo se serializovat manifest: {}", e);
        error!("{}", msg);
        msg
    })?;

    fs::write(&manifest_path, content).map_err(|e| {
        let msg = format!(
            "Nepodařilo se zapsat manifest '{}': {}",
            manifest_path.display(),
            e
        );
        error!("{}", msg);
        msg
    })?;

    info!("[save_manifest] Manifest uložen: {}", manifest_path.display());
    Ok(())
}

#[tauri::command]
async fn extract_package(
    download_path: String,
    slug: String,
) -> Result<String, String> {
    let game_dir = Path::new(&download_path).join(&slug);
    let zip_path = game_dir.join("temp.zip");
    let content_dir = game_dir.join("content");

    info!(
        "[extract_package] Rozbaluji '{}' do '{}'",
        zip_path.display(),
        content_dir.display()
    );

    // Vytvoření cílové složky content/
    fs::create_dir_all(&content_dir).map_err(|e| {
        let msg = format!(
            "Nepodařilo se vytvořit složku '{}': {}",
            content_dir.display(),
            e
        );
        error!("{}", msg);
        msg
    })?;

    // Otevření ZIP archivu
    let zip_file = fs::File::open(&zip_path).map_err(|e| {
        let msg = format!("Nepodařilo se otevřít ZIP '{}': {}", zip_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    let mut archive = zip::ZipArchive::new(zip_file).map_err(|e| {
        let msg = format!("Nepodařilo se přečíst ZIP archiv: {}", e);
        error!("{}", msg);
        msg
    })?;

    // Rozbalení všech souborů
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            let msg = format!("Chyba při čtení položky ze ZIPu (index {}): {}", i, e);
            error!("{}", msg);
            msg
        })?;

        let out_path = match file.enclosed_name() {
            Some(path) => content_dir.join(path),
            None => {
                warn!("[extract_package] Přeskakuji nebezpečnou cestu v ZIPu (index {})", i);
                continue;
            }
        };

        if file.is_dir() {
            trace!("[extract_package] Vytvářím složku: {}", out_path.display());
            fs::create_dir_all(&out_path).map_err(|e| {
                let msg = format!("Nepodařilo se vytvořit složku '{}': {}", out_path.display(), e);
                error!("{}", msg);
                msg
            })?;
        } else {
            // Zajisti že nadřazené složky existují
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    let msg = format!("Nepodařilo se vytvořit složku '{}': {}", parent.display(), e);
                    error!("{}", msg);
                    msg
                })?;
            }
            trace!("[extract_package] Rozbaluji soubor: {}", out_path.display());
            let mut out_file = fs::File::create(&out_path).map_err(|e| {
                let msg = format!("Nepodařilo se vytvořit soubor '{}': {}", out_path.display(), e);
                error!("{}", msg);
                msg
            })?;
            io::copy(&mut file, &mut out_file).map_err(|e| {
                let msg = format!("Chyba při zápisu souboru '{}': {}", out_path.display(), e);
                error!("{}", msg);
                msg
            })?;
        }
    }

    info!(
        "[extract_package] Rozbaleno {} položek do '{}'",
        archive.len(),
        content_dir.display()
    );

    // Smazání temp.zip
    fs::remove_file(&zip_path).map_err(|e| {
        let msg = format!("Nepodařilo se smazat temp ZIP '{}': {}", zip_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    info!("[extract_package] Smazán dočasný soubor: {}", zip_path.display());

    Ok(content_dir.to_string_lossy().to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct InstalledPackage {
    slug: String,
    #[serde(rename = "type")]
    package_type: String,
    title: String,
    description: String,
    file_size: u64,
    archive_file: String,
    created_at: Option<String>,
    #[serde(default = "default_status")]
    status: String,
    /// Absolutní cesta ke složce balíčku ({download_path}/{slug})
    package_path: String,
}

#[tauri::command]
async fn scan_library(download_path: String) -> Result<Vec<InstalledPackage>, String> {
    info!("[scan_library] Skenuji složku: {}", download_path);

    let base = Path::new(&download_path);
    if !base.exists() {
        warn!("[scan_library] Složka neexistuje: {}", download_path);
        return Ok(vec![]);
    }

    let mut packages: Vec<InstalledPackage> = Vec::new();

    // Hloubka 1 = přímé podsložky (úroveň 2 = soubory v nich)
    for entry in WalkDir::new(base)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() != "manifest.json" {
            continue;
        }

        let manifest_path = entry.path();
        debug!("[scan_library] Nalezen manifest: {}", manifest_path.display());

        let content = match fs::read_to_string(manifest_path) {
            Ok(c) => c,
            Err(e) => {
                warn!(
                    "[scan_library] Nelze přečíst manifest '{}': {}",
                    manifest_path.display(),
                    e
                );
                continue;
            }
        };

        // Nejprve parse na ManifestData (manifest.json neobsahuje package_path)
        let manifest: ManifestData = match serde_json::from_str(&content) {
            Ok(m) => m,
            Err(e) => {
                warn!(
                    "[scan_library] Nepodařilo se parsovat manifest '{}': {}",
                    manifest_path.display(),
                    e
                );
                continue;
            }
        };

        // Vytvoř InstalledPackage s doplněnou package_path
        let pkg_dir = manifest_path
            .parent()
            .unwrap_or(base)
            .to_string_lossy()
            .to_string();

        let installed = InstalledPackage {
            slug: manifest.slug,
            package_type: manifest.package_type,
            title: manifest.title,
            description: manifest.description,
            file_size: manifest.file_size,
            archive_file: manifest.archive_file,
            created_at: manifest.created_at,
            status: manifest.status,
            package_path: pkg_dir,
        };

        packages.push(installed);
    }

    info!("[scan_library] Nalezeno {} balíčků", packages.len());
    Ok(packages)
}

#[tauri::command]
async fn update_manifest_status(
    package_path: String,
    status: String,
) -> Result<(), String> {
    // Validace status hodnoty
    if status != "installing" && status != "ready" {
        return Err(format!("Neplatný status '{}': povoleno pouze 'installing' nebo 'ready'", status));
    }

    let manifest_path = Path::new(&package_path).join("manifest.json");

    info!(
        "[update_manifest_status] Aktualizuji status na '{}' v '{}'",
        status,
        manifest_path.display()
    );

    let content = fs::read_to_string(&manifest_path).map_err(|e| {
        let msg = format!("Nelze přečíst manifest '{}': {}", manifest_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    let mut json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        let msg = format!("Nelze parsovat manifest '{}': {}", manifest_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    json["status"] = serde_json::Value::String(status.clone());

    let new_content = serde_json::to_string_pretty(&json).map_err(|e| {
        let msg = format!("Nelze serializovat manifest: {}", e);
        error!("{}", msg);
        msg
    })?;

    fs::write(&manifest_path, new_content).map_err(|e| {
        let msg = format!("Nelze zapsat manifest '{}': {}", manifest_path.display(), e);
        error!("{}", msg);
        msg
    })?;

    info!(
        "[update_manifest_status] Status aktualizován na '{}': {}",
        status,
        manifest_path.display()
    );
    Ok(())
}

#[tauri::command]
async fn open_folder(package_path: String) -> Result<(), String> {
    let content_dir = Path::new(&package_path).join("content");
    info!("[open_folder] Otevírám složku: {}", content_dir.display());

    if !content_dir.exists() {
        let msg = format!("Složka neexistuje: {}", content_dir.display());
        error!("{}", msg);
        return Err(msg);
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&content_dir)
            .spawn()
            .map_err(|e| {
                let msg = format!("Nepodařilo se otevřít Průzkumník: {}", e);
                error!("{}", msg);
                msg
            })?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&content_dir)
            .spawn()
            .map_err(|e| {
                let msg = format!("Nepodařilo se otevřít Finder: {}", e);
                error!("{}", msg);
                msg
            })?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&content_dir)
            .spawn()
            .map_err(|e| {
                let msg = format!("Nepodařilo se otevřít správce souborů: {}", e);
                error!("{}", msg);
                msg
            })?;
    }

    Ok(())
}

#[tauri::command]
async fn uninstall_package(
    download_path: String,
    slug: String,
) -> Result<(), String> {
    info!("[uninstall_package] Odstraňuji balíček: {}", slug);

    // Validace slug
    if slug.trim().is_empty() || slug.contains('/') || slug.contains('\\') || slug.contains("..") {
        return Err("Neplatný slug".to_string());
    }
    for c in slug.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err("Neplatný slug: povoleny pouze A-Z, a-z, 0-9, '-', '_' a '.'".to_string());
        }
    }

    // Canonicalize základní cesty
    fs::create_dir_all(&download_path).map_err(|e| {
        format!("Nepodařilo se vytvořit základní složku: {}", e)
    })?;
    let base_canon = fs::canonicalize(&download_path).map_err(|e| {
        format!("Nepodařilo se canonicalizovat download_path: {}", e)
    })?;

    let package_dir = base_canon.join(&slug);

    // Bezpečnostní kontrola – musí být pod download_path
    if !package_dir.starts_with(&base_canon) {
        let msg = format!(
            "Bezpečnostní chyba: cílová složka '{}' není pod '{}'",
            package_dir.display(),
            base_canon.display()
        );
        error!("{}", msg);
        return Err(msg);
    }

    if !package_dir.exists() {
        let msg = format!("Složka balíčku neexistuje: {}", package_dir.display());
        warn!("{}", msg);
        return Err(msg);
    }

    fs::remove_dir_all(&package_dir).map_err(|e| {
        let msg = format!(
            "Nepodařilo se odstranit složku '{}': {}",
            package_dir.display(),
            e
        );
        error!("{}", msg);
        msg
    })?;

    info!("[uninstall_package] Odstraněno: {}", package_dir.display());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(if cfg!(debug_assertions) {
                    tauri_plugin_log::log::LevelFilter::Trace // MARK: TRACING
                } else {
                    tauri_plugin_log::log::LevelFilter::Trace // TODO: pro final release změnit
                })
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: None,
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .filter(|metadata| {
                    // Tao hází WARN logy na focus lost/gain
                    if metadata.target().starts_with("tao") {
                        return metadata.level() <= log::Level::Error;
                    }
                    true
                })
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            download_package,
            save_manifest,
            extract_package,
            scan_library,
            update_manifest_status,
            open_folder,
            uninstall_package,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
