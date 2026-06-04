// Structured audit-event trail. Indexers (e.g. a MiCA audit log) consume these
// to reconstruct the full authorization history of an asset off-chain. Defined
// with the soroban-sdk 26 `#[contractevent]` macro so the event name is the
// first topic and `account` is indexed.
use soroban_sdk::{contractevent, Address, Env};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorized {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deauthorized {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Banned {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unbanned {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allowed {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Disallowed {
    #[topic]
    pub account: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paused {}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unpaused {}

// Thin helpers so the contract body reads cleanly.
pub fn authorized(env: &Env, account: &Address) {
    Authorized {
        account: account.clone(),
    }
    .publish(env);
}
pub fn deauthorized(env: &Env, account: &Address) {
    Deauthorized {
        account: account.clone(),
    }
    .publish(env);
}
pub fn banned(env: &Env, account: &Address) {
    Banned {
        account: account.clone(),
    }
    .publish(env);
}
pub fn unbanned(env: &Env, account: &Address) {
    Unbanned {
        account: account.clone(),
    }
    .publish(env);
}
pub fn allowed(env: &Env, account: &Address) {
    Allowed {
        account: account.clone(),
    }
    .publish(env);
}
pub fn disallowed(env: &Env, account: &Address) {
    Disallowed {
        account: account.clone(),
    }
    .publish(env);
}
pub fn paused(env: &Env) {
    Paused {}.publish(env);
}
pub fn unpaused(env: &Env) {
    Unpaused {}.publish(env);
}
