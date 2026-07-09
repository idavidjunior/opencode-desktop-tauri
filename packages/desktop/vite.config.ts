import { defineConfig } from "vite"
import appPlugin from "@opencode-ai/app/vite"

export default defineConfig({
  root: "src/renderer",
  base: "./",
  publicDir: "../../../app/public",
  plugins: [appPlugin],
  build: {
    outDir: "../../out/renderer",
    emptyOutDir: true,
    sourcemap: true,
    target: "esnext",
    rollupOptions: {
      input: "src/renderer/index.html",
    },
  },
  server: {
    port: 5173,
    strictPort: true,
  },
})
