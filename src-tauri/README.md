# PasteHaste

A global hotkey paste utility for Windows. Assign text to slots and paste them anywhere with configurable keyboard shortcuts. Also supports random address generation and account lookups.

Built with [Tauri v2](https://v2.tauri.app/) (Rust + HTML/JS frontend).

## Features

- **Paste slots** &mdash; 5 configurable slots, each with a paste shortcut (default `Alt+1` through `Alt+5`) and a copy-in shortcut (default `Ctrl+Alt+1` through `Ctrl+Alt+5`).
- **Address generator** &mdash; generate a random address and paste individual fields (street, city, state, zip) via hotkeys.
- **System tray** &mdash; runs in the tray with Show/Hide, Enable/Disable, and Quit controls.
- **Persistent storage** &mdash; slot contents and shortcuts are saved to `slots.json` in the app data directory.

## Prerequisites

### Rust toolchain

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or on Windows, download and run [rustup-init.exe](https://win.rustup.rs/).

Verify the installation:

```bash
rustc --version
cargo --version
```

### Node.js

Install [Node.js](https://nodejs.org/) (LTS recommended, v18+). This provides `npm` which is used to run the Tauri CLI.

### Windows system dependencies

Tauri v2 on Windows requires:

- **WebView2** &mdash; pre-installed on Windows 10 (1803+) and Windows 11. If missing, download the [Evergreen Bootstrapper](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).
- **Visual Studio Build Tools** &mdash; install the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload selected. This provides the MSVC compiler and Windows SDK.

## Setup

Clone the repository and install the JavaScript dependencies:

```bash
git clone <repo-url>
cd pastehaste
npm install
```

## Development

Run the app in development mode with hot-reload:

```bash
npm run dev
```

This compiles the Rust backend, launches the app window, and watches the frontend for changes.

## Building

Create a production build:

```bash
npm run build
```

The bundled installer (NSIS `.exe`) will be output to `src-tauri/target/release/bundle/nsis/`.

## Usage

### Paste slots

Each of the 5 slots has a **paste** and **copy** shortcut. By default:

| Slot | Paste       | Copy into slot |
|------|-------------|----------------|
| 1    | `Alt+1`     | `Ctrl+Alt+1`   |
| 2    | `Alt+2`     | `Ctrl+Alt+2`   |
| 3    | `Alt+3`     | `Ctrl+Alt+3`   |
| 4    | `Alt+4`     | `Ctrl+Alt+4`   |
| 5    | `Alt+5`     | `Ctrl+Alt+5`   |

- Type or paste text into a slot's input field, then press the paste shortcut in any application to type it out.
- Press the copy shortcut to capture the current clipboard selection into the slot.
- Shortcuts are fully configurable from the UI.

### Address generator

Generate a random address and bind hotkeys to paste individual address fields (street, city, state, zip) into any application.

### System tray

Closing the window hides it to the tray. Right-click the tray icon for:

- **Show/Hide** &mdash; toggle the main window.
- **Enable/Disable** &mdash; toggle all hotkeys on or off.
- **Quit** &mdash; save state and exit.

## Legacy AHK version

The original AutoHotkey v2 script is available at `pastehaste.ahk` for standalone use without the Tauri app.

## Project structure

```
pastehaste/
├── src/                  # Frontend (HTML/JS)
│   └── index.html
├── src-tauri/
│   ├── Cargo.toml        # Rust dependencies
│   ├── tauri.conf.json   # Tauri app configuration
│   ├── capabilities/     # Tauri permission declarations
│   └── src/
│       ├── lib.rs        # App entry point and Tauri setup
│       ├── main.rs       # Binary entry point
│       ├── slots.rs      # Slot state management and commands
│       ├── hotkeys.rs    # Global shortcut registration
│       ├── input.rs      # Simulated keyboard input (Win32)
│       └── address.rs    # Random address generation
├── pastehaste.ahk        # Legacy AutoHotkey script
└── package.json          # npm scripts (tauri dev / build)
```
