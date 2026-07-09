# OpenCode Desktop

The OpenCode Desktop app, built with [Tauri](https://v2.tauri.app/) v2 (Rust) and a
SolidJS renderer. The Node.js server from `packages/opencode` runs as a sidecar
child process; the Rust core drives the window, menus, dialogs, store, updater,
and deep links.

## Architecture

- `src/` — SolidJS renderer (unchanged from the original Electron build).
- `src-tauri/` — Rust application:
  - `src/cmds/` — Tauri commands invoked from the renderer via `window.api.*`
    (see `src-tauri/api-shim.js`, the preload replacement).
  - `src/server.rs` — finds a free port and spawns the Node.js sidecar.
  - `src/store.rs` — on-disk JSON key/value store (electron-store compatible).
- `scripts/prebuild.ts` — generates icons, metainfo, and the `node.js` build
  from `packages/opencode`, plus a `sidecar-entry.mjs` entry point and a
  `node_modules` symlink so the ESM sidecar can resolve externalized deps
  (`@lydell/node-pty`, `jsonc-parser`).

## Development

```bash
bun install
bun run tauri:dev
```

`tauri:dev` runs the Vite renderer dev server and launches the Rust app with a
hot-reloading sidecar.

## Build

```bash
bun run tauri:build
```

Produces a platform-native bundle in `src-tauri/target/release/bundle/`.

## Notes

- The desktop reads `window.api.*` from `src-tauri/api-shim.js` injected at
  runtime — the renderer is identical to the legacy Electron build.
- The updater uses `tauri-plugin-updater`; the dev signing key in
  `tauri.conf.json` must be replaced with the production key before release.
- Node.js sidecar dependencies are externalized and resolved at runtime via a
  `node_modules` symlink created by `scripts/prebuild.ts` (dev) or bundled as
  resources (release).
