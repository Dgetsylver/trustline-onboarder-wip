import type { OnboarderConfig } from "@theaha/authline";

/**
 * Config-driven. The live asset + network come from PUBLIC_* env (app/.env or
 * the Pages workflow). Add/replace the live asset by config alone.
 */

export const NETWORK = {
  rpcUrl: import.meta.env.PUBLIC_STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org",
  horizonUrl: import.meta.env.PUBLIC_STELLAR_HORIZON_URL ?? "https://horizon-testnet.stellar.org",
  passphrase:
    import.meta.env.PUBLIC_STELLAR_NETWORK_PASSPHRASE ?? "Test SDF Network ; September 2015",
  allowHttp: false,
};

export const NETWORK_LABEL = NETWORK.passphrase.includes("Public")
  ? "Stellar · Mainnet"
  : "Stellar · Testnet";

const CODE = import.meta.env.PUBLIC_ASSET_CODE ?? "EURCV";

/** The live, wired asset (the one the dApp actually activates on-chain). */
export interface AssetConfig extends OnboarderConfig {
  name: string;
  glyph: string;
  kind: string;
  networkLabel: string;
}

export const ASSET: AssetConfig = {
  assetCode: CODE,
  assetIssuer:
    import.meta.env.PUBLIC_ASSET_ISSUER ?? "GCEYGIVOLAVBF2TG2RUSGTUJCIN75KEX3NGLMY4VPL4GFE5L355AXW3G",
  sac: import.meta.env.PUBLIC_SAC ?? "",
  authorizer: import.meta.env.PUBLIC_AUTHORIZER ?? "",
  onboard: import.meta.env.PUBLIC_ONBOARD,
  backends: ["cap73-one-signature", "cap33-sponsored"],
  name: import.meta.env.PUBLIC_ASSET_NAME ?? "Stellar asset",
  glyph: CODE.slice(0, 2).toUpperCase(),
  kind: import.meta.env.PUBLIC_ASSET_KIND ?? "Stellar asset",
  networkLabel: NETWORK_LABEL,
};

/** Directory: the configured asset is Live; the rest are the roadmap. */
export interface DirItem {
  code: string;
  name: string;
  glyph: string;
  kind: string;
  status: "live" | "soon";
}

export const ASSETS: DirItem[] = [
  { code: ASSET.assetCode, name: ASSET.name, glyph: ASSET.glyph, kind: ASSET.kind, status: "live" },
  { code: "USDC", name: "USD Coin", glyph: "US", kind: "USD stablecoin", status: "soon" },
  { code: "BENJI", name: "Franklin MMF", glyph: "BE", kind: "Tokenized treasuries", status: "soon" },
  { code: "EURC", name: "Euro Coin", glyph: "EC", kind: "Euro stablecoin", status: "soon" },
];

export const REPO_URL = "https://github.com/Dgetsylver/trustline-onboarder-wip";
