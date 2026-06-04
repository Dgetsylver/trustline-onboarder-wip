#![cfg(test)]
extern crate std;

use super::{Policy, TrustlineAuthorizer, TrustlineAuthorizerClient};
use soroban_sdk::testutils::{Address as _, IssuerFlags};
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::{Address, Env, Vec};

/// Register an AUTH_REQUIRED SAC and a TrustlineAuthorizer set as its admin.
/// Returns (env, issuer/admin, sac address, authorizer address).
fn setup(policy: Policy) -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let sac_addr = sac.address();
    // Regulated-asset flag set: AUTH_REQUIRED (gate trustlines), AUTH_REVOCABLE
    // (freeze/deauthorize), AUTH_CLAWBACK_ENABLED (claw back) — the MiCA control set.
    sac.issuer().set_flag(IssuerFlags::RequiredFlag);
    sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    sac.issuer().set_flag(IssuerFlags::ClawbackEnabledFlag);

    // Deploy the authorizer (admin = issuer) and hand it the SAC admin role.
    let authorizer = env.register(
        TrustlineAuthorizer,
        (issuer.clone(), sac_addr.clone(), policy),
    );
    StellarAssetClient::new(&env, &sac_addr).set_admin(&authorizer);

    (env, issuer, sac_addr, authorizer)
}

fn one(env: &Env, a: &Address) -> Vec<Address> {
    Vec::from_array(env, [a.clone()])
}

#[test]
fn denylist_authorizes_unbanned_holder() {
    let (env, _issuer, sac_addr, authorizer) = setup(Policy::Denylist);
    let client = TrustlineAuthorizerClient::new(&env, &authorizer);
    let sac = StellarAssetClient::new(&env, &sac_addr);
    let holder = Address::generate(&env);

    sac.trust(&holder); // CAP-73: create the (unauthorized) trustline
    assert!(!sac.authorized(&holder));

    client.authorize_trustline(&holder);
    assert!(sac.authorized(&holder));
}

#[test]
fn denylist_blocks_banned_holder() {
    let (env, _issuer, sac_addr, authorizer) = setup(Policy::Denylist);
    let client = TrustlineAuthorizerClient::new(&env, &authorizer);
    let sac = StellarAssetClient::new(&env, &sac_addr);
    let holder = Address::generate(&env);

    sac.trust(&holder);
    client.ban(&one(&env, &holder));
    assert!(client.is_banned(&holder));

    let res = client.try_authorize_trustline(&holder);
    assert!(res.is_err());
    assert!(!sac.authorized(&holder));
}

#[test]
fn freeze_then_unfreeze_roundtrip() {
    let (env, _issuer, sac_addr, authorizer) = setup(Policy::Denylist);
    let client = TrustlineAuthorizerClient::new(&env, &authorizer);
    let sac = StellarAssetClient::new(&env, &sac_addr);
    let holder = Address::generate(&env);

    sac.trust(&holder);
    client.authorize_trustline(&holder);
    assert!(sac.authorized(&holder));

    // Freeze == ban + deauthorize.
    client.freeze(&one(&env, &holder));
    assert!(client.is_banned(&holder));
    assert!(!sac.authorized(&holder));

    // A retried self-service authorization MUST stay blocked while banned.
    assert!(client.try_authorize_trustline(&holder).is_err());
    assert!(!sac.authorized(&holder));

    // Unfreeze == unban + re-authorize.
    client.unfreeze(&one(&env, &holder));
    assert!(!client.is_banned(&holder));
    assert!(sac.authorized(&holder));
}

#[test]
fn allowlist_gates_on_explicit_allow() {
    let (env, _issuer, sac_addr, authorizer) = setup(Policy::Allowlist);
    let client = TrustlineAuthorizerClient::new(&env, &authorizer);
    let sac = StellarAssetClient::new(&env, &sac_addr);
    let holder = Address::generate(&env);

    sac.trust(&holder);

    // Not yet allowed -> rejected.
    assert!(client.try_authorize_trustline(&holder).is_err());
    assert!(!sac.authorized(&holder));

    // After allow -> authorized.
    client.allow(&one(&env, &holder));
    client.authorize_trustline(&holder);
    assert!(sac.authorized(&holder));
}

#[test]
fn pause_blocks_authorization() {
    let (env, _issuer, sac_addr, authorizer) = setup(Policy::Denylist);
    let client = TrustlineAuthorizerClient::new(&env, &authorizer);
    let sac = StellarAssetClient::new(&env, &sac_addr);
    let holder = Address::generate(&env);

    sac.trust(&holder);
    client.pause();
    assert!(client.try_authorize_trustline(&holder).is_err());

    client.unpause();
    client.authorize_trustline(&holder);
    assert!(sac.authorized(&holder));
}
