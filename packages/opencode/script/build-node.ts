#!/usr/bin/env bun

import { $ } from "bun"
import { Script } from "@opencode-ai/script"
import path from "path"
import { fileURLToPath } from "url"
import * as fs from "fs"

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const dir = path.resolve(__dirname, "..")

process.chdir(dir)

const generated = await import("./generate.ts")

const nodeDist = path.join(dir, "dist", "node")
await fs.promises.mkdir(nodeDist, { recursive: true })

await Bun.build({
  target: "node",
  entrypoints: ["./src/node.ts"],
  outdir: "./dist/node",
  format: "esm",
  sourcemap: "linked",
  external: [],
  define: {
    OPENCODE_MODELS_DEV: generated.modelsData,
    OPENCODE_CHANNEL: `'${Script.channel}'`,
  },
  files: {
    "opencode-web-ui.gen.ts": "",
  },
  // Bundle all dependencies inline
  conditions: ["node"],
})

console.log("Build complete")
