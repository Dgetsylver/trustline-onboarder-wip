# Trustline Onboarder — common tasks. Run `just` for the list.

default:
    @just --list

# Run all contract tests (native).
test:
    cargo test

# Build deployable wasm for both contracts.
build-contracts:
    cargo build --release --target wasm32v1-none

# Lint + format check.
check:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

# Format all Rust code.
fmt:
    cargo fmt

# Build the integrator SDK.
sdk:
    npm --prefix packages/sdk run build

# Run the activation page locally.
app:
    npm --prefix app run dev
