use soroban_sdk::contracttype;

/// Authorization policy for an asset's trustlines.
///
/// - `Denylist` (default, frictionless): every trustline is auto-authorized on
///   request **unless** the holder is banned. This is the EURCV model — a
///   permissionless regulated stablecoin where onboarding is open by default
///   and compliance acts by exception (freeze / ban).
/// - `Allowlist` (gated): a trustline is authorized **only** if the holder has
///   been explicitly allowed first (e.g. tokenized securities requiring per-user
///   KYC before holding).
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Policy {
    Denylist,
    Allowlist,
}
