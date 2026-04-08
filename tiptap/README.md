# Bundling the Tiptap JS Snippets

Install the pinned dependencies from the committed lockfile

    npm ci

This project uses two separate flows:

- `just build` for reproducible rebuilds from `package-lock.json`
- `just update-tiptap` when intentionally upgrading the npm dependencies and regenerating the checked-in bundle
- `npm run typecheck` for bridge-level TypeScript validation
- `npm test` for bridge-level unit tests

Bundle the bridge runtime, separated Tiptap core runtime, and official extension snippets with esbuild

    npm run build

The generated JS files

    ../leptos-tiptap/src/js/generated/

contain:

- `bridge_runtime.js`: the Rust-facing bridge runtime and editor registry
- `tiptap_core.js`: the bundled Tiptap and ProseMirror runtime modules
- `tiptap_*.js`: standalone official Tiptap extension registration modules

These generated files are imported from the Rust crate through `wasm-bindgen` local JS modules and are copied into final application build output automatically. They are not meant to be served manually from a consumer project.

The TypeScript source for this package lives under

    src/
