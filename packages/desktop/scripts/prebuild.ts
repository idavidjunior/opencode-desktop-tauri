#!/usr/bin/env bun
import { $ } from "bun"
import * as path from "node:path"
import * as fs from "node:fs/promises"
import { resolveChannel } from "./utils"

const channel = resolveChannel()
await $`bun ./scripts/copy-icons.ts ${channel}`
await $`bun ./scripts/copy-metainfo.ts ${channel}`

// Build the node sidecar
await $`cd ../opencode && bun script/build-node.ts`

// Copy node_modules so the ESM sidecar can resolve external deps (@lydell/node-pty)
// Filter out .d.ts and .map files that cause WiX build failures
const nodeDist = path.resolve(import.meta.dir, "../../opencode/dist/node")
const desktopNodeModules = path.resolve(import.meta.dir, "../node_modules")
const serverNodeModules = path.join(nodeDist, "node_modules")

// Remove existing node_modules (symlink or dir)
try {
  await fs.rm(serverNodeModules, { recursive: true, force: true })
} catch {}

// Copy node_modules to node_dist for bundling in the installer
// Filter out .d.ts and .map files that cause WiX build failures (LGHT0103)
async function copyFiltered(src: string, dest: string) {
  await fs.mkdir(dest, { recursive: true })
  const entries = await fs.readdir(src, { withFileTypes: true })
  for (const entry of entries) {
    const srcPath = path.join(src, entry.name)
    const destPath = path.join(dest, entry.name)
    if (entry.isDirectory()) {
      await copyFiltered(srcPath, destPath)
    } else if (entry.isFile() && !entry.name.endsWith(".d.ts") && !entry.name.endsWith(".map")) {
      await fs.copyFile(srcPath, destPath)
    }
  }
}

await copyFiltered(desktopNodeModules, serverNodeModules)

// Write sidecar entry point (reads env vars, starts server)
await fs.writeFile(
  path.join(nodeDist, "sidecar-entry.mjs"),
  `import { Server } from "./node.js"

const port = parseInt(process.env.OPENCODE_PORT, 10)
if (!port) throw new Error("OPENCODE_PORT not set")

Server.listen({
  port,
  hostname: process.env.OPENCODE_HOSTNAME || "127.0.0.1",
  username: process.env.OPENCODE_SERVER_USERNAME || "opencode",
  password: process.env.OPENCODE_SERVER_PASSWORD || "",
  cors: ["oc://renderer"],
})
`,
)
