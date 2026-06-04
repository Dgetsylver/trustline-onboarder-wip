/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly PUBLIC_STELLAR_RPC_URL?: string;
  readonly PUBLIC_STELLAR_HORIZON_URL?: string;
  readonly PUBLIC_STELLAR_NETWORK_PASSPHRASE?: string;
  readonly PUBLIC_DISCOVER_DOMAIN?: string;
  readonly PUBLIC_TITLE?: string;
  readonly PUBLIC_SUBTITLE?: string;
  readonly PUBLIC_ACCENT?: string;
  readonly PUBLIC_ASSET_CODE?: string;
  readonly PUBLIC_ASSET_ISSUER?: string;
  readonly PUBLIC_SAC?: string;
  readonly PUBLIC_AUTHORIZER?: string;
  readonly PUBLIC_ONBOARD?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
