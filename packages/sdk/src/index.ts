/**
 * @theaha/trustline-onboarder
 *
 * Integrator SDK for one-signature activation of AUTH_REQUIRED Stellar assets.
 * Wallets and exchanges use this to embed the Trustline Onboarder flow:
 * discover an issuer's onboarder config from its stellar.toml, build the
 * one-signature `onboard` transaction, and check activation status.
 *
 * See the SEP draft: ../../sep/SEP-XXXX-trustline-onboarder.md
 */

export type Backend = "cap73-one-signature" | "cap33-sponsored";

/** Resolved onboarder configuration for a single asset. */
export interface OnboarderConfig {
  /** Classic asset code, e.g. "EURCV". */
  assetCode: string;
  /** Classic asset issuer (G...). */
  assetIssuer: string;
  /** The asset's Stellar Asset Contract (C...). */
  sac: string;
  /** The Trustline Authorizer contract (the SAC admin) (C...). */
  authorizer: string;
  /** The one-signature onboard wrapper contract (C...), if deployed. */
  onboard?: string;
  /** Backends the issuer supports, in preference order. */
  backends: Backend[];
}

export { discoverOnboarder, discoverOnboarder as discover, parseOnboarderToml } from "./discovery.js";
export { buildOnboardTx, type BuildOnboardOptions } from "./onboard.js";
export { getActivationStatus, getActivationStatus as status, assetAuthRequired, type ActivationStatus } from "./status.js";
// Third-party (exchange / broker / wallet) integration surface.
export {
  buildAuthorizeTx,
  buildSponsoredOnboardTx,
  onboardingRequest,
  asAccount,
  type OnboardingRequest,
} from "./exchange.js";

/**
 * Pick the backend to use for a given holder. The CAP-73 one-signature path is
 * preferred when the wrapper is deployed and the holder already has a funded,
 * on-ledger account (CAP-73 `trust()` has no sponsorship — the holder pays the
 * trustline reserve). Otherwise fall back to the CAP-33 sponsored path.
 */
export function selectBackend(
  config: { onboard?: string; backends: Backend[] },
  holder: { exists: boolean; fundedForReserve: boolean },
): Backend {
  const canOneSig =
    !!config.onboard &&
    config.backends.includes("cap73-one-signature") &&
    holder.exists &&
    holder.fundedForReserve;
  if (canOneSig) return "cap73-one-signature";
  return "cap33-sponsored";
}
