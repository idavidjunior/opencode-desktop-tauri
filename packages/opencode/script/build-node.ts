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

// Find the compiled .node file - use require.resolve to locate the package
let nodeFilePath = ""
let nodeFile = ""

try {
  // Try to resolve the module from the temp directory
  const resolved = require.resolve("@lydell/node-pty", { paths: [nativeModuleDir] })
  const resolvedDir = path.dirname(resolved)
  
  // Look for .node file in the resolved directory and subdirectories
  const findNodeFile = async (dirPath: string): Promise<string | null> => {
    const entries = await fs.promises.readdir(dirPath, { withFileTypes: true })
    for (const entry of entries) {
      const fullPath = path.join(dirPath, entry.name)
      if (entry.isDirectory()) {
        const found = await findNodeFile(fullPath)
        if (found) return found
      } else if (entry.name.endsWith(".node")) {
        return fullPath
      }
    }
    return null
  }
  
  nodeFilePath = await findNodeFile(resolvedDir) || ""
  if (nodeFilePath) {
    nodeFile = path.basename(nodeFilePath)
  }
} catch {}

// Fallback: check Bun's internal cache structure
if (!nodeFilePath) {
  const bunCacheDir = path.join(nativeModuleDir, "node_modules", ".bun")
  if (await fs.promises.stat(bunCacheDir).catch(() => false)) {
    const findInBunCache = async (dirPath: string): Promise<string | null> => {
      const entries = await fs.promises.readdir(dirPath, { withFileTypes: true })
      for (const entry of entries) {
        const fullPath = path.join(dirPath, entry.name)
        if (entry.isDirectory()) {
          const found = await findInBunCache(fullPath)
          if (found) return found
        } else if (entry.name.endsWith(".node")) {
          return fullPath
        }
      }
      return null
    }
    nodeFilePath = await findInBunCache(bunCacheDir) || ""
    if (nodeFilePath) {
      nodeFile = path.basename(nodeFilePath)
    }
  }
}

if (!nodeFilePath) {
  throw new Error("Could not find .node file for @lydell/node-pty")
}

const nodeDist = path.join(dir, "dist", "node")
await fs.promises.mkdir(nodeDist, { recursive: true })

// Copy the .node file to dist/node
const targetNodePath = path.join(nodeDist, "node-pty.node")
await fs.promises.copyFile(nodeFilePath, targetNodePath)
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
