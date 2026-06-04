import type { Backend, OnboarderConfig } from "./index.js";

/**
 * Discover an issuer's Trustline Onboarder support from its domain's
 * `stellar.toml` (SEP-1). The issuer advertises a `[TRUSTLINE_ONBOARDER]`
 * block, for example:
 *
 * ```toml
 * [TRUSTLINE_ONBOARDER]
 * ASSET_CODE = "EURCV"
 * ASSET_ISSUER = "GCEYGIVOLAVBF2TG2RUSGTUJCIN75KEX3NGLMY4VPL4GFE5L355AXW3G"
 * SAC = "C..."
 * AUTHORIZER = "C..."          # the Trustline Authorizer (SAC admin)
 * ONBOARD = "C..."             # the one-signature CAP-73 wrapper
 * BACKENDS = ["cap73-one-signature", "cap33-sponsored"]
 * ```
 */
export async function discoverOnboarder(
  domain: string,
  fetchImpl: typeof fetch = fetch,
): Promise<OnboarderConfig | null> {
  const url = `https://${domain.replace(/\/$/, "")}/.well-known/stellar.toml`;
  const res = await fetchImpl(url);
  if (!res.ok) return null;
  return parseOnboarderToml(await res.text());
}

/** Parse the `[TRUSTLINE_ONBOARDER]` block out of a stellar.toml document. */
export function parseOnboarderToml(toml: string): OnboarderConfig | null {
  const block = sectionBody(toml, "TRUSTLINE_ONBOARDER");
  if (!block) return null;

  const assetCode = str(block, "ASSET_CODE");
  const assetIssuer = str(block, "ASSET_ISSUER");
  const sac = str(block, "SAC");
  const authorizer = str(block, "AUTHORIZER");
  const onboard = str(block, "ONBOARD");
  const backends = arr(block, "BACKENDS").filter(isBackend);

  if (!assetCode || !assetIssuer || !sac || !authorizer) return null;
  return {
    assetCode,
    assetIssuer,
    sac,
    authorizer,
    onboard: onboard || undefined,
    backends: backends.length ? backends : ["cap73-one-signature", "cap33-sponsored"],
  };
}

function sectionBody(toml: string, name: string): string | null {
  const lines = toml.split(/\r?\n/);
  const start = lines.findIndex((l) => l.trim() === `[${name}]`);
  if (start < 0) return null;
  const body: string[] = [];
  for (let i = start + 1; i < lines.length; i++) {
    if (/^\s*\[/.test(lines[i])) break;
    body.push(lines[i]);
  }
  return body.join("\n");
}

function str(block: string, key: string): string {
  const m = block.match(new RegExp(`^\\s*${key}\\s*=\\s*"([^"]*)"`, "m"));
  return m ? m[1] : "";
}
function arr(block: string, key: string): string[] {
  const m = block.match(new RegExp(`^\\s*${key}\\s*=\\s*\\[([^\\]]*)\\]`, "m"));
  if (!m) return [];
  return m[1]
    .split(",")
    .map((s) => s.trim().replace(/^"|"$/g, ""))
    .filter(Boolean);
}
function isBackend(s: string): s is Backend {
  return s === "cap73-one-signature" || s === "cap33-sponsored";
}
