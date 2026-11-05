#!/usr/bin/env bun

import { $ } from "bun"
import { Script } from "@opencode-ai/script"
import path from "path"
import { fileURLToPath } from "url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const dir = path.resolve(__dirname, "..")

process.chdir(dir)

const generated = await import("./generate.ts")

await Bun.build({
  target: "node",
  entrypoints: ["./src/node.ts"],
  outdir: "./dist/node",
  format: "esm",
  sourcemap: "linked",
  external: ["@lydell/node-pty"],
  define: {
    OPENCODE_MODELS_DEV: generated.modelsData,
    OPENCODE_CHANNEL: `'${Script.channel}'`,
  },
  files: {
    "opencode-web-ui.gen.ts": "",
  },
  // Bundle all dependencies inline except node-pty
  conditions: ["node"],
})

// Install @lydell/node-pty in the dist/node folder so it gets bundled in the Tauri installer
const nodeDist = path.join(dir, "dist", "node")
console.log("Installing @lydell/node-pty in dist/node...")
await $`bun install @lydell/node-pty --cwd ${nodeDist}`

console.log("Build complete")
