#![cfg(test)]
extern crate std;

use super::{TrustlineOnboard, TrustlineOnboardClient};
use soroban_sdk::testutils::{Address as _, IssuerFlags};
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::{Address, Env};
use trustline_authorizer::{Policy, TrustlineAuthorizer};

/// End-to-end: one `onboard()` call creates the trustline (CAP-73) and
/// authorizes it through the **real** Trustline Authorizer (the SAC admin).
#[test]
fn onboard_creates_trustline_and_authorizes() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let issuer = Address::generate(&env);
    let holder = Address::generate(&env);

    // AUTH_REQUIRED SAC.
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let sac_addr = sac.address();
    sac.issuer().set_flag(IssuerFlags::RequiredFlag);

    // Real authorizer (denylist) set as the SAC admin.
    let authorizer = env.register(
        TrustlineAuthorizer,
        (issuer.clone(), sac_addr.clone(), Policy::Denylist),
    );
    StellarAssetClient::new(&env, &sac_addr).set_admin(&authorizer);

    // One-signature onboarding from a fresh holder.
    let onboard = env.register(TrustlineOnboard, ());
    let client = TrustlineOnboardClient::new(&env, &onboard);
    client.onboard(&sac_addr, &authorizer, &holder);

    assert!(StellarAssetClient::new(&env, &sac_addr).authorized(&holder));
}

/// A banned holder cannot be onboarded: the authorize step reverts the whole tx.
#[test]
fn onboard_reverts_for_banned_holder() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let issuer = Address::generate(&env);
    let holder = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let sac_addr = sac.address();
    sac.issuer().set_flag(IssuerFlags::RequiredFlag);
    sac.issuer().set_flag(IssuerFlags::RevocableFlag); // ban deauthorizes

    let authorizer = env.register(
        TrustlineAuthorizer,
        (issuer.clone(), sac_addr.clone(), Policy::Denylist),
    );
    StellarAssetClient::new(&env, &sac_addr).set_admin(&authorizer);

    // Holder has a trustline, then gets banned (ban = deauthorize + denylist).
    StellarAssetClient::new(&env, &sac_addr).trust(&holder);
    trustline_authorizer::TrustlineAuthorizerClient::new(&env, &authorizer)
        .ban(&soroban_sdk::Vec::from_array(&env, [holder.clone()]));

    // A retried onboard must revert at the authorize step and leave the holder
    // unauthorized.
    let onboard = env.register(TrustlineOnboard, ());
    let client = TrustlineOnboardClient::new(&env, &onboard);

    assert!(client.try_onboard(&sac_addr, &authorizer, &holder).is_err());
    assert!(!StellarAssetClient::new(&env, &sac_addr).authorized(&holder));
}
