# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

# Lists all available commands.
stable_all_features := "component,full,ssr"

# Install the tools this crate depends on for local development.
install-tools:
  cargo install just
  cargo install cargo-leptos
  cargo install wasm-bindgen-cli
  cargo install cargo-audit
  cargo install cargo-deny
  cargo install cargo-semver-checks
  cargo install leptosfmt

# Perform a full build of the tiptap bundle.
build:
    just install-tiptap
    just bundle-tiptap

# Install pinned tiptap and the required JS build tooling.
install-tiptap:
    cd tiptap && npm ci

# Upgrade every official Tiptap dependency to one exact, synchronized version.
# The default selector resolves to the newest stable Tiptap 2 release.
update-tiptap version="2":
    cd tiptap && node update-tiptap.mjs {{ version }}
    cd tiptap && npm install
    just bundle-tiptap

# Bundle the Rust-facing Tiptap host runtime and standalone extension modules into
# src/js/generated/.
bundle-tiptap:
    cd tiptap && npm run build

# Run the core validation suite, including static bridge parity, generated-bundle
# drift, and runtime WASM ABI checks.
verify:
    cargo fmt --all -- --check
    cd tiptap && npm test
    cd tiptap && npm run typecheck
    cd tiptap && npm run build:check
    cargo check --no-default-features
    cargo check --no-default-features --features starter-kit
    cargo clippy -- -D warnings
    cargo clippy --all-targets --no-default-features --features {{stable_all_features}} -- -D warnings
    cargo test --lib
    cargo test --doc
    cargo test --lib --features full
    cargo test --doc --features full
    just wasm-abi-test
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features --features {{stable_all_features}}
    cargo test --test browser_test -- --nocapture
    cargo build --features ssr
    cargo build --target wasm32-unknown-unknown --features full
    cargo check --manifest-path examples/demo-csr/Cargo.toml --target wasm32-unknown-unknown
    cargo check --manifest-path examples/demo-ssr/Cargo.toml --features ssr

# Exercise the real serde_wasm_bindgen request and response shapes in JavaScript.
wasm-abi-test:
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --lib --target wasm32-unknown-unknown --no-default-features --features full

# Verify the nightly-only Leptos integrations without requiring nightly for the stable suite.
verify-nightly:
    cargo +nightly check --no-default-features --features nightly
    cargo +nightly check --no-default-features --features component,nightly
    cargo +nightly check --target wasm32-unknown-unknown --no-default-features --features component,full,nightly

# Run the Chrome-based browser integration test headlessly.
browser-test:
    cargo test --test browser_test -- --nocapture

# Run the Chrome-based browser integration test with a visible browser.
browser-test-visible:
    BROWSER_TEST_VISIBLE=1 cargo test --test browser_test -- --nocapture

# Start the SSR demo and pause before running browser assertions.
browser-test-pause:
    BROWSER_TEST_VISIBLE=1 BROWSER_TEST_PAUSE=1 cargo test --test browser_test -- --nocapture

# Run cargo-deny's supply-chain checks (advisories, bans, licenses, sources).
deny:
    cargo deny check

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv find --path .
