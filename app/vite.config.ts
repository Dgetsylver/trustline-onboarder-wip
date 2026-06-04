import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Static SPA — deployable to Cloudflare Pages / IPFS with no backend.
export default defineConfig({
  plugins: [react()],
  envPrefix: "PUBLIC_",
  // Relative base so the build works under a GitHub Pages project subpath
  // (https://<user>.github.io/<repo>/) and survives a repo rename.
  base: "./",
  // Stellar Wallets Kit (and some deps) reference Node globals in the browser.
  define: { global: "globalThis" },
});
