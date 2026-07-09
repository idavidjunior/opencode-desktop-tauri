# Desktop (Tauri)

- Electron replaced by Tauri v2 (Rust)
- Renderer process uses `window.api` via `api-shim.js` (injected by Tauri at runtime)
- All IPC handlers are Rust commands in `src-tauri/src/cmds/`
- Sidecar: Node.js server spawned as child process (replaces Electron `utilityProcess`)
