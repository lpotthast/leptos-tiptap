# AGENTS.md

Guidance for Codex when working in this repository.

## Repo Snapshot

- `leptos-tiptap/` is the main library crate.
- `tiptap/` is the Node.js bundle source used to produce the browser-side Tiptap bundle.
- The example apps live under `leptos-tiptap/examples/`, not at the repository root.

Current versions in the repo:

- `leptos-tiptap`: `0.10.0`
- `leptos`: `0.8.2`
- Tiptap npm packages in `tiptap/package.json`: `2.27.2`

## Common Commands

Rebuild the Tiptap JS bundle:

```sh
just build
```

Build the library crate:

```sh
cd leptos-tiptap && cargo build
```

Build the library crate with SSR feature:

```sh
cd leptos-tiptap && cargo build --features ssr
```

Run the CSR example (requires `cargo install trunk`):

```sh
cd leptos-tiptap/examples/demo-csr && trunk serve
```

Run the SSR example (requires `cargo install cargo-leptos`):

```sh
cd leptos-tiptap/examples/demo-ssr && cargo leptos watch
```

Find the MSRV used by the crates:

```sh
just msrv
```

## Architecture

This repository wraps the [Tiptap](https://tiptap.dev/) editor for [Leptos](https://leptos.dev/).

## Testing Notes

- Use the `assertr` crate for Rust test assertions instead of the standard `assert!` / `assert_eq!` macros.

There are two layers:

1. `leptos-tiptap`
   The runtime crate. It exposes the `TiptapInstance` component, `TiptapEditorHandle`, content/resource types, and selection state types.
2. `tiptap/`
   The Node.js source for the crate-local JS snippets. `just build` bundles the bridge runtime, the separated Tiptap core runtime, and standalone extension modules into `leptos-tiptap/src/js/generated/`.

## Important Files

- `leptos-tiptap/src/lib.rs`
  Public types such as `TiptapContent`, `TiptapSelectionState`, `TiptapHeadingLevel`, and resource structs.
- `leptos-tiptap/src/tiptap_instance.rs`
  The `TiptapInstance` component. This is the main Rust-side lifecycle and command dispatch code.
- `leptos-tiptap/src/js_tiptap.rs`
  The `wasm_bindgen` FFI layer. All browser interaction goes through this file.
- `tiptap/src/bridge_runtime.ts`
  The bridge-owned JS runtime that manages editor lifecycle, commands, and selection state.
- `tiptap/src/tiptap_core.ts`
  The Tiptap/ProseMirror runtime bundle that is initialized before the bridge runtime.

## Runtime Model

- `TiptapInstance` is not a fully controlled component.
  The `initial_content` prop is treated as one-time editor initialization input. Callers should not continuously mirror user edits back into `initial_content`.
- The `id` prop must be globally unique across all editor instances.
  It is a stable DOM id, not a reset mechanism.
- Commands are driven through `TiptapEditorHandle` methods obtained from `on_ready` or `on_change`.
- Selection updates come back through `on_selection_change`.
- Content updates are lightweight notifications through `on_change`.
  Consumers use the `TiptapEditorHandle` from `on_ready` or `on_change` to pull the current HTML or JSON content on demand.
- Full-document content replacement is done explicitly through `TiptapEditorHandle`.
- A single editor instance can be read back as both HTML and JSON.

## SSR and CSR Behavior

- JS interop in `leptos-tiptap/src/js_tiptap.rs` is wrapped in `cfg_if!`.
- With the `ssr` feature enabled, all JS calls become no-ops.
- `TiptapInstance` still renders its DOM node on the server and hydrates on the client.
- The runtime imports crate-local `wasm-bindgen` JS snippets, which are copied into the final app build output automatically.

## JS Integration Notes

- The Rust bridge imports generated crate-local JS snippets such as `bridge_runtime.js`, `tiptap_core.js`, and `tiptap_*.js`.
- The editor registry lives inside the bridge runtime module.
- If you add or remove commands on the Rust side, update both `leptos-tiptap/src/js_tiptap.rs` and the relevant `tiptap/src/bridge_*.ts` or `tiptap/src/extensions/tiptap_*.ts` modules.
- If you add a new extension, update `tiptap/package.json`, `tiptap/build.mjs`, and add a new `tiptap/src/extensions/tiptap_*.ts` module.

## Compatibility

| leptos-tiptap | Leptos | Tiptap |
|---------------|--------|--------|
| 0.9.x         | 0.8.x  | 2.12.x |
| 0.8.x         | 0.7.x  | —      |
| 0.7.x         | 0.6.x  | —      |
