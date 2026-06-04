#![no_std]
//! # Trustline Authorizer
//!
//! Asset-agnostic generalization of the live `eurcv_auth` contract
//! (`theahaco/eurcv_auth`, mainnet `CB2DHZMQHQE3TGUMD6BRM7UCJZNIPKDRVEQOWBIRRS3G2FZOGDTRKSB3`).
//!
//! The issuer of an `AUTH_REQUIRED` classic asset sets this contract as the
//! admin of the asset's Stellar Asset Contract (SAC) via `set_admin`. This
//! delegates the authorize-on-behalf decision to predictable, auditable
//! on-chain policy instead of an off-chain signer.
//!
//! Policy is configurable per asset:
//! - **Denylist** (default, frictionless): `authorize_trustline` is permissionless
//!   and auto-authorizes any holder that is not banned.
//! - **Allowlist** (gated): `authorize_trustline` only authorizes holders that
//!   were explicitly allowed first.
//!
//! Freeze is implemented as **ban + deauthorize** so that a retried onboarding
//! (e.g. via the `trustline-onboard` wrapper) can never re-authorize a frozen
//! holder — see `ban` / `freeze`.
//!
//! The admin interface (`admin` / `set_admin` / `upgrade`) follows the Aha team's
//! Contract Admin SEP (`theahaco/admin-sep`).

mod events;
mod policy;
mod storage;

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contracterror, contractimpl, token::StellarAssetClient, Address, BytesN, Env, Vec,
};

pub use policy::Policy;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The contract is paused; authorization is temporarily disabled.
    Paused = 1,
    /// The account is on the denylist.
    AccountBanned = 2,
    /// The account is not on the allowlist (allowlist policy only).
    AccountNotAllowed = 3,
}

#[contract]
pub struct TrustlineAuthorizer;

#[contractimpl]
impl TrustlineAuthorizer {
    /// Initialize the authorizer. The issuer must, in a separate issuer-signed
    /// step, set this contract as the SAC admin (`SAC.set_admin(this)`).
    pub fn __constructor(env: Env, admin: Address, sac: Address, policy: Policy) {
        storage::set_admin(&env, &admin);
        storage::set_sac(&env, &sac);
        storage::set_policy(&env, &policy);
        storage::set_paused(&env, false);
    }

    // --- authorization (permissionless self-service) -----------------------

    /// Request authorization of `account`'s trustline.
    ///
    /// Permissionless: anyone can trigger it (matching `eurcv_auth`), but the
    /// policy is re-checked on **every** call, so a banned (denylist) or
    /// not-allowed (allowlist) account is never authorized. The contract — which
    /// is the SAC admin — performs the authorization via `SAC.set_authorized`.
    pub fn authorize_trustline(env: Env, account: Address) -> Result<(), Error> {
        if storage::is_paused(&env) {
            return Err(Error::Paused);
        }
        match storage::get_policy(&env) {
            Policy::Denylist => {
                if storage::is_banned(&env, &account) {
                    return Err(Error::AccountBanned);
                }
            }
            Policy::Allowlist => {
                if !storage::is_allowed(&env, &account) {
                    return Err(Error::AccountNotAllowed);
                }
            }
        }
        sac(&env).set_authorized(&account, &true);
        events::authorized(&env, &account);
        Ok(())
    }

    /// Admin: revoke a trustline's authorization without banning the holder.
    pub fn deauthorize_trustline(env: Env, account: Address) {
        require_admin(&env);
        sac(&env).set_authorized(&account, &false);
        events::deauthorized(&env, &account);
    }

    // --- denylist (and freeze) ---------------------------------------------

    /// Admin: ban accounts. Banning also **deauthorizes** the trustline, so this
    /// is the freeze primitive: a banned account cannot self-re-authorize.
    pub fn ban(env: Env, accounts: Vec<Address>) {
        require_admin(&env);
        let sac = sac(&env);
        for account in accounts.iter() {
            storage::set_banned(&env, &account, true);
            sac.set_authorized(&account, &false);
            events::banned(&env, &account);
        }
    }

    /// Admin: remove accounts from the denylist. Does **not** re-authorize; call
    /// `unfreeze` (or have the holder call `authorize_trustline` again) for that.
    pub fn unban(env: Env, accounts: Vec<Address>) {
        require_admin(&env);
        for account in accounts.iter() {
            storage::set_banned(&env, &account, false);
            events::unbanned(&env, &account);
        }
    }

    /// Admin: freeze = ban + deauthorize. Alias of `ban` for call-site clarity.
    pub fn freeze(env: Env, accounts: Vec<Address>) {
        Self::ban(env, accounts);
    }

    /// Admin: unfreeze = unban + re-authorize.
    pub fn unfreeze(env: Env, accounts: Vec<Address>) {
        require_admin(&env);
        let sac = sac(&env);
        for account in accounts.iter() {
            storage::set_banned(&env, &account, false);
            sac.set_authorized(&account, &true);
            events::unbanned(&env, &account);
            events::authorized(&env, &account);
        }
    }

    // --- allowlist ---------------------------------------------------------

    /// Admin: add accounts to the allowlist (allowlist policy).
    pub fn allow(env: Env, accounts: Vec<Address>) {
        require_admin(&env);
        for account in accounts.iter() {
            storage::set_allowed(&env, &account, true);
            events::allowed(&env, &account);
        }
    }

    /// Admin: remove accounts from the allowlist and deauthorize them.
    pub fn disallow(env: Env, accounts: Vec<Address>) {
        require_admin(&env);
        let sac = sac(&env);
        for account in accounts.iter() {
            storage::set_allowed(&env, &account, false);
            sac.set_authorized(&account, &false);
            events::disallowed(&env, &account);
        }
    }

    // --- supply controls (MiCA: issuer must be able to mint / claw back) ----

    pub fn mint(env: Env, to: Address, amount: i128) {
        require_admin(&env);
        sac(&env).mint(&to, &amount);
    }

    pub fn clawback(env: Env, from: Address, amount: i128) {
        require_admin(&env);
        sac(&env).clawback(&from, &amount);
    }

    // --- pause -------------------------------------------------------------

    pub fn pause(env: Env) {
        require_admin(&env);
        storage::set_paused(&env, true);
        events::paused(&env);
    }

    pub fn unpause(env: Env) {
        require_admin(&env);
        storage::set_paused(&env, false);
        events::unpaused(&env);
    }

    // --- admin interface (admin-sep) ---------------------------------------

    pub fn admin(env: Env) -> Address {
        storage::get_admin(&env)
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        require_admin(&env);
        storage::set_admin(&env, &new_admin);
    }

    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        require_admin(&env);
        env.deployer().update_current_contract_wasm(wasm_hash);
    }

    // --- views -------------------------------------------------------------

    pub fn policy(env: Env) -> Policy {
        storage::get_policy(&env)
    }
    pub fn sac_address(env: Env) -> Address {
        storage::get_sac(&env)
    }
    pub fn is_banned(env: Env, account: Address) -> bool {
        storage::is_banned(&env, &account)
    }
    pub fn is_allowed(env: Env, account: Address) -> bool {
        storage::is_allowed(&env, &account)
    }
    pub fn is_paused(env: Env) -> bool {
        storage::is_paused(&env)
    }
}

fn require_admin(env: &Env) {
    storage::get_admin(env).require_auth();
}

fn sac(env: &Env) -> StellarAssetClient<'_> {
    StellarAssetClient::new(env, &storage::get_sac(env))
}
