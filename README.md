# Trustline Onboarder

One-signature activation of `AUTH_REQUIRED` Stellar classic assets.

Receiving a Stellar classic asset normally forces the user through a context-free
*"create a trustline"* prompt and a separate authorization step. This collapses
that into **one signature**: a Soroban wrapper chains CAP-73 `SAC.trust()` (create
the trustline) and an on-chain authorization policy (`set_authorized`) under a
single `holder.require_auth()`.

## How it works

```
        ┌──────────┐   trust()  (CAP-73, Protocol 26)   ┌─────────────────────┐
holder  │ onboard()│ ─────────────────────────────────▶ │  Stellar Asset      │
signs ─▶│  wrapper │                                     │  Contract (SAC)     │
 once   │          │ ─ authorize_trustline() ─┐          └─────────┬───────────┘
        └──────────┘                          ▼                    │ set_authorized
                                   ┌──────────────────────┐        │
                                   │ Trustline Authorizer │◀───────┘  (SAC admin)
                                   │ denylist / allowlist │
                                   └──────────────────────┘
```

1. The issuer keeps the asset `AUTH_REQUIRED` and sets the **Trustline Authorizer**
   as the SAC admin (`SAC.set_admin`).
2. The holder signs **one** `onboard(sac, authorizer, holder)` transaction.
3. `onboard` runs `SAC.trust(holder)` (CAP-73 creates the trustline) then
   `authorizer.authorize_trustline(holder)`, which — gated by the denylist /
   allowlist policy — calls `SAC.set_authorized(holder, true)`.
4. The whole invocation is atomic: any failure reverts.

CAP-73's `trust()` has no sponsorship, so this single-signature path is for a
**funded** holder. A reserve-free path for a brand-new account uses classic
sponsored reserves (CAP-33).

## Contracts

| Contract | Description |
|---|---|
| [`trustline-authorizer`](contracts/trustline-authorizer) | SAC-admin authorization policy. `authorize_trustline` (permissionless, policy-gated), `ban`/`unban`, `freeze`/`unfreeze` (= ban + deauthorize), `allow`/`disallow`, `deauthorize_trustline`, `mint`, `clawback`, `pause`/`unpause`, admin (`admin`/`set_admin`/`upgrade`). Emits an audit-event trail. Denylist (frictionless) or allowlist (gated) policy. |
| [`trustline-onboard`](contracts/trustline-onboard) | One-signature CAP-73 wrapper: `onboard(sac, authorizer, holder)`. |

## Packages

- [`packages/sdk`](packages/sdk) — `@theaha/trustline-onboarder`: integrator SDK
  (`discoverOnboarder`, `buildOnboardTx`, `getActivationStatus`, React `useActivation`/`ActivateButton`).
- [`app`](app) — a config-driven activation page (Vite + React + Stellar Wallets Kit).

## Build & test

```bash
# contracts: native unit/integration tests
cargo test

# contracts: build deployable wasm
cargo build --release --target wasm32v1-none

# lint
cargo clippy --all-targets && cargo fmt --check

# SDK
npm install
npm run build --workspace @theaha/trustline-onboarder

# activation page (dev)
npm run dev --workspace trustline-onboarder-app
```

## Project structure

```
.
├── contracts/
│   ├── trustline-authorizer/   # SAC-admin authorization policy (Rust/Soroban)
│   └── trustline-onboard/      # one-signature CAP-73 wrapper (Rust/Soroban)
├── packages/sdk/               # @theaha/trustline-onboarder (TypeScript SDK)
├── app/                        # activation page (Vite + React)
└── environments.toml           # Scaffold Stellar network/deploy config
```

## License

[Apache-2.0](LICENSE).
