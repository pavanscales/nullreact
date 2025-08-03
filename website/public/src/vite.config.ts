import { defineConfig } from "vite"

export default defineConfig({
  root: ".",
  publicDir: "public",
  build: {
    outDir: "dist",
    rollupOptions: {
      input: "public/playground.html"
    }
  }
})
