use soroban_sdk::{contracttype, Address, Env};

use crate::policy::Policy;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// The admin (issuer or its delegate) authorized to manage policy.
    Admin,
    /// The Stellar Asset Contract this authorizer administers.
    Sac,
    /// The active authorization policy (denylist / allowlist).
    Policy,
    /// Pause flag — when true, `authorize_trustline` is rejected.
    Paused,
    /// Denylist membership for `account`.
    Banned(Address),
    /// Allowlist membership for `account`.
    Allowed(Address),
}

// --- instance config -------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}
pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}
pub fn set_sac(env: &Env, sac: &Address) {
    env.storage().instance().set(&DataKey::Sac, sac);
}
pub fn get_sac(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Sac).unwrap()
}
pub fn set_policy(env: &Env, policy: &Policy) {
    env.storage().instance().set(&DataKey::Policy, policy);
}
pub fn get_policy(env: &Env) -> Policy {
    env.storage().instance().get(&DataKey::Policy).unwrap()
}
pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}
pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

// --- denylist / allowlist sets (persistent) --------------------------------

pub fn set_banned(env: &Env, account: &Address, banned: bool) {
    let key = DataKey::Banned(account.clone());
    if banned {
        env.storage().persistent().set(&key, &true);
    } else {
        env.storage().persistent().remove(&key);
    }
}
pub fn is_banned(env: &Env, account: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Banned(account.clone()))
        .unwrap_or(false)
}

pub fn set_allowed(env: &Env, account: &Address, allowed: bool) {
    let key = DataKey::Allowed(account.clone());
    if allowed {
        env.storage().persistent().set(&key, &true);
    } else {
        env.storage().persistent().remove(&key);
    }
}
pub fn is_allowed(env: &Env, account: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Allowed(account.clone()))
        .unwrap_or(false)
}
