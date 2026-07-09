import { Server } from "./node.js"

const port = parseInt(process.env.OPENCODE_PORT, 10)
if (!port) throw new Error("OPENCODE_PORT not set")

await Server.listen({
  port,
  hostname: process.env.OPENCODE_HOSTNAME || "127.0.0.1",
  username: process.env.OPENCODE_SERVER_USERNAME || "opencode",
  password: process.env.OPENCODE_SERVER_PASSWORD || "",
  cors: ["oc://renderer"],
})
