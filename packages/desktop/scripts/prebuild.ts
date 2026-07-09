#!/usr/bin/env bun
import { $ } from "bun"
import * as path from "node:path"
import * as fs from "node:fs/promises"

import { resolveChannel } from "./utils"

const channel = resolveChannel()
await $`bun ./scripts/copy-icons.ts ${channel}`
await $`bun ./scripts/copy-metainfo.ts ${channel}`

await $`cd ../opencode && bun script/build-node.ts`

// Symlink node_modules so the ESM sidecar can resolve external deps (@lydell/node-pty)
const nodeDist = path.resolve(import.meta.dir, "../../opencode/dist/node")
const desktopNodeModules = path.resolve(import.meta.dir, "../node_modules")
const serverNodeModules = path.join(nodeDist, "node_modules")
await fs.symlink(desktopNodeModules, serverNodeModules, "dir").catch(() => {})

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
