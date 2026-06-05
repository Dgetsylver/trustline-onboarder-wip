#![no_std]
//! # Trustline Onboard (one-signature, CAP-73)
//!
//! Asset-agnostic generalization of the `trustline-onboard` wrapper proposed in
//! `theahaco/stellar-assets` PR #10 (https://github.com/theahaco/stellar-assets/pull/10).
//!
//! It chains, under a **single** `holder.require_auth()`:
//!   1. `SAC.trust(holder)` — CAP-73 (Stellar Protocol 26): create the holder's
//!      trustline through the Stellar Asset Contract; and
//!   2. `Authorizer.authorize_trustline(holder)` — the [`trustline-authorizer`]
//!      (the SAC admin) authorizes it via `SAC.set_authorized`.
//!
//! This collapses the legacy two-transaction flow (classic `ChangeTrust` +
//! a separate Soroban authorize call) into one Soroban transaction and one
//! signature. CAP-73's `trust()` requires the holder to sign and to pay the
//! 0.5 XLM trustline reserve (CAP-73 has no sponsorship), so this path is for a
//! **funded** holder (e.g. a CEX-withdrawal recipient). The reserve-free path
//! for a brand-new account uses classic sponsored reserves (CAP-33) off-chain.

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, token::StellarAssetClient, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// `SAC.trust` failed (e.g. issuer missing).
    TrustFailed = 1,
    /// The authorizer rejected the trustline (banned / not allowed / paused).
    AuthorizationFailed = 2,
    /// Post-condition failed: the holder is still not authorized on `sac` after
    /// the authorize call (a wrong/divergent SAC, or an authorizer that no-ops).
    NotAuthorized = 3,
}

/// The minimal interface the onboard wrapper needs from a Trustline Authorizer.
/// Satisfied by `trustline-authorizer` (and by the live `eurcv_auth` contract).
#[contractclient(name = "AuthorizerClient")]
pub trait Authorizer {
    fn authorize_trustline(env: Env, account: Address) -> Result<(), soroban_sdk::Error>;
}

#[contract]
pub struct TrustlineOnboard;

#[contractimpl]
impl TrustlineOnboard {
    /// Create-and-authorize `holder`'s trustline for the asset behind `sac`,
    /// where `authorizer` is the SAC admin. One signature (the holder's); the
    /// whole invocation reverts on any inner failure.
    pub fn onboard(
        env: Env,
        sac: Address,
        authorizer: Address,
        holder: Address,
    ) -> Result<(), Error> {
        holder.require_auth();

        // CAP-73: idempotent — a no-op if the trustline already exists.
        let sac_client = StellarAssetClient::new(&env, &sac);
        sac_client
            .try_trust(&holder)
            .map_err(|_| Error::TrustFailed)?
            .map_err(|_| Error::TrustFailed)?;

        AuthorizerClient::new(&env, &authorizer)
            .try_authorize_trustline(&holder)
            .map_err(|_| Error::AuthorizationFailed)?
            .map_err(|_| Error::AuthorizationFailed)?;

        // Post-condition: confirm the holder is actually authorized on THIS sac.
        // Guards against a wrong/divergent SAC or an authorizer that no-ops
        // (matches the hardening in stellar-assets PR #10).
        if !sac_client.authorized(&holder) {
            return Err(Error::NotAuthorized);
        }

        Ok(())
    }
}
