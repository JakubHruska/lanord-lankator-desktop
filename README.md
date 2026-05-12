# Lanord Manager - Desktop Client

Lanord Manager je desktopová aplikace pro správu, stahování a instalaci softwarových a herních balíčků. Aplikace
komunikuje s API serverem na lokální síti, stahuje archivy s balíčky a poskytuje uživatelské rozhraní pro lokální knihovnu
stažených titulů.

Projekt je postaven na stacku: **Tauri 2**, **Vue 3** a **PrimeVue**. O operace na úrovni systému (bezpečné
stahování velkých souborů na disk, extrakce ZIP archivů) se stará backend v jazyce **Rust**.

---

## 🛠️ Prerekvizity pro sestavení

Pro kompilaci a spuštění ze zdrojových kódů potřebujete mít nainstalováno:

1. **Node.js** (ideálně verze 18/20 LTS) a balíčkovací manažer `npm`.
2. **Rust** a nástroj `cargo` (snadná instalace přes [rustup.rs](https://rustup.rs/)).
3. **Systémové závislosti pro Tauri**:
    - *Windows*: Visual Studio C++ Build Tools a instalační sadu Windows 10/11 SDK.
    - *Linux / macOS*: Specifické systémové knihovny najdete
      v [oficiální dokumentaci Tauri](https://v2.tauri.app/start/prerequisites/).

---

## 🚀 Uvedení do provozu a sestavení (Build)

Otevřete terminál v kořenové složce projektu (`desktop_client`) a vložte následující příkazy:

1. **Instalace závislostí pro frontend:**
   ```bash
   npm install
   ```

2. **Spuštění ve vývojovém režimu (nepovinné):**
   Pro spuštění aplikace s live-reloadem (Vue Dev server + Tauri okno):
   ```bash
   npm run tauri dev
   ```

3. **Vytvoření finálního produkčního buildu:**
   Zkompiluje frontend přes Vite a spustí kompilaci RUST binárky s optimizacemi:
   ```bash
   npm run tauri build
   ```

---

## 📦 Kde najít výslednou aplikaci

Pokud příkaz `npm run tauri build` proběhne bez chyb a aplikaci úspěšně zkompiluje, najdete finální release soubory ve
struktuře `src-tauri/target/release/`:

- **Samostatný spustitelný soubor (.exe na Windows)** najdete na cestě:
  `src-tauri/target/release/desktop_client.exe`

- **Finální Instalátor (např. NSIS instalátor)** pro distribuci uživatelům najdete v automaticky vygenerované složce:
  `src-tauri/target/release/bundle/nsis/`
