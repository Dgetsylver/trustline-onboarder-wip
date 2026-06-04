import { dirname, resolve } from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [react()],
  envPrefix: "PUBLIC_",
  // Relative base so it works under a GitHub Pages project subpath and survives a rename.
  base: "./",
  // Stellar Wallets Kit (and some deps) reference Node globals in the browser.
  define: { global: "globalThis" },
  build: {
    rollupOptions: {
      input: {
        main: resolve(root, "index.html"), // landing page
        app: resolve(root, "app.html"), // activation dApp
      },
    },
  },
});
