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

# Bundle the Rust-facing Tiptap adapter into a minified ESM file -> leptos-tiptap-build/dist/tiptap.js
bundle-tiptap:
  mkdir -p leptos-tiptap-build/dist
  cd tiptap && npm run build

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv find --path leptos-tiptap
    cargo msrv find --path leptos-tiptap-build
