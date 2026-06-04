import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Static SPA — deployable to Cloudflare Pages / IPFS with no backend.
export default defineConfig({
  plugins: [react()],
  envPrefix: "PUBLIC_",
  // Stellar Wallets Kit (and some deps) reference Node globals in the browser.
  define: { global: "globalThis" },
});
