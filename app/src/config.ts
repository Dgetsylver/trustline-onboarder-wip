import type { OnboarderConfig } from "@theaha/trustline-onboarder";

/**
 * Config-driven activation page. Add a new asset by editing this file (or by
 * pointing `DISCOVER_DOMAIN` at an issuer whose stellar.toml carries a
 * `[TRUSTLINE_ONBOARDER]` block). No code changes required.
 */

export const NETWORK = {
  rpcUrl: import.meta.env.PUBLIC_STELLAR_RPC_URL ?? "https://mainnet.sorobanrpc.com",
  horizonUrl: import.meta.env.PUBLIC_STELLAR_HORIZON_URL ?? "https://horizon.stellar.org",
  passphrase:
    import.meta.env.PUBLIC_STELLAR_NETWORK_PASSPHRASE ??
    "Public Global Stellar Network ; September 2015",
};

/** Optional: discover config live from an issuer domain instead of hardcoding. */
export const DISCOVER_DOMAIN: string | null = import.meta.env.PUBLIC_DISCOVER_DOMAIN ?? null;

export const BRANDING = {
  title: import.meta.env.PUBLIC_TITLE ?? "Welcome to Stellar",
  subtitle: import.meta.env.PUBLIC_SUBTITLE ?? "Activate your asset in one signature.",
  accent: import.meta.env.PUBLIC_ACCENT ?? "#47a019",
};

/** Static fallback config (the live EURCV precursor; replace SAC/ONBOARD once deployed). */
export const ASSET: OnboarderConfig = {
  assetCode: import.meta.env.PUBLIC_ASSET_CODE ?? "EURCV",
  assetIssuer:
    import.meta.env.PUBLIC_ASSET_ISSUER ?? "GCEYGIVOLAVBF2TG2RUSGTUJCIN75KEX3NGLMY4VPL4GFE5L355AXW3G",
  sac: import.meta.env.PUBLIC_SAC ?? "C_REPLACE_WITH_EURCV_SAC",
  authorizer:
    import.meta.env.PUBLIC_AUTHORIZER ?? "CB2DHZMQHQE3TGUMD6BRM7UCJZNIPKDRVEQOWBIRRS3G2FZOGDTRKSB3",
  onboard: import.meta.env.PUBLIC_ONBOARD,
  backends: ["cap73-one-signature", "cap33-sponsored"],
};
