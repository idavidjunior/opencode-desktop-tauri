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

// First, build the native module (@lydell/node-pty) to get the .node file
console.log("Building @lydell/node-pty native module...")
const nativeModuleDir = path.join(dir, "temp-node-pty")
await $`mkdir -p ${nativeModuleDir}`
await $`cd ${nativeModuleDir} && bun add @lydell/node-pty`

// Find the compiled .node file
const nodePtyPath = path.join(nativeModuleDir, "node_modules", "@lydell", "node-pty")
const nodeFiles = await fs.promises.readdir(nodePtyPath, { recursive: true })
const nodeFile = nodeFiles.find(f => f.endsWith(".node"))

if (!nodeFile) {
  throw new Error("Could not find .node file for @lydell/node-pty")
}

const nodeFilePath = path.join(nodePtyPath, nodeFile)
const nodeDist = path.join(dir, "dist", "node")
await fs.promises.mkdir(nodeDist, { recursive: true })

// Copy the .node file to dist/node
const targetNodePath = path.join(nodeDist, "node-pty.node")
await fs.promises.copyFile(path.join(nodePtyPath, nodeFile), targetNodePath)
console.log(`Copied ${nodeFile} to ${targetNodePath}`)

// Clean up temp directory
await fs.promises.rm(nativeModuleDir, { recursive: true, force: true })

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
    "node-pty.node": await fs.promises.readFile(targetNodePath),
  },
  // Bundle all dependencies inline
  conditions: ["node"],
})

console.log("Build complete")
