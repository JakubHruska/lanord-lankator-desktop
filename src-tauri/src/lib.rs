use log::{debug, error, info, warn};
use reqwest;
use std::fs;
use std::path::Path;

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

    let bytes = response.bytes().await.map_err(|e| {
        let msg = format!("Chyba při čtení dat: {}", e);
        error!("{}", msg);
        msg
    })?;

    if bytes.is_empty() {
        warn!("[download_package] Server vrátil prázdný soubor");
        return Err("Server vrátil prázdný soubor".to_string());
    }

    info!("[download_package] Staženo {} bajtů", bytes.len());

    // 3. Uložení ZIPu na disk
    let zip_path = game_dir.join(format!("{}.zip", slug));
    fs::write(&zip_path, &bytes).map_err(|e| {
        let msg = format!(
            "Nepodařilo se zapsat soubor '{}': {}",
            zip_path.display(),
            e
        );
        error!("{}", msg);
        msg
    })?;

    info!("[download_package] Uloženo do: {}", zip_path.display());
    Ok(zip_path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(if cfg!(debug_assertions) {
                    tauri_plugin_log::log::LevelFilter::Debug
                } else {
                    tauri_plugin_log::log::LevelFilter::Info // TODO: pro final release změnit
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, download_package])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
