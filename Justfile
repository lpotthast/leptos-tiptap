# Lists all available commands.
list:
  just --list

# Perform a full build of the tiptap bundle.
build:
  just install-tiptap
  just bundle-tiptap

# Install pinned tiptap and the required JS build tooling.
install-tiptap:
  cd tiptap && npm ci

# Explicitly upgrade the tiptap npm dependencies and refresh the bundle.
update-tiptap:
  cd tiptap && npm update
  just bundle-tiptap

# Bundle the Rust-facing Tiptap host runtime and standalone extension modules into
# leptos-tiptap/src/js/generated/.
bundle-tiptap:
  cd tiptap && npm run build

# Run the core validation suite, including generated-bundle drift checks.
verify:
  cd tiptap && npm test
  cd tiptap && npm run typecheck
  cd tiptap && npm run build:check
  cd leptos-tiptap && cargo test
  cd leptos-tiptap && cargo test --features full
  cd leptos-tiptap && cargo build --features ssr
  cd leptos-tiptap && cargo build --target wasm32-unknown-unknown --features full
  cd leptos-tiptap && cargo clippy --all-targets --all-features -- -D warnings

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv find --path leptos-tiptap
