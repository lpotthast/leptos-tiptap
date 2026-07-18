# Bundling the Tiptap JS Snippets

Install the pinned dependencies from the committed lockfile

    npm ci

This project uses two separate flows:

- `just build` for reproducible rebuilds from `package-lock.json`
- `just update-tiptap` when intentionally upgrading the npm dependencies and regenerating the checked-in bundle
- `npm run build:check` to rebuild and fail if the generated bundle changed
- `npm run typecheck` for bridge-level TypeScript validation
- `npm test` for bridge-level unit tests

The generated browser modules target ES2022. Type checking is split by execution environment:

- `tsconfig.browser.json` checks the shipped bridge and extension sources with browser globals only
- `tsconfig.tests.json` adds Node types for the TypeScript test suite
- `tsconfig.tooling.json` checks the Node-based `.mjs` build and maintenance scripts without DOM globals

Bundle the bridge runtime and official extension snippets with esbuild

    npm run build

The generated JS files

    ../src/js/generated/

contain:

- `bridge_runtime.js`: the Rust-facing bridge runtime, editor registry, and shared Tiptap/ProseMirror base
- `tiptap_*.js`: standalone official Tiptap extension registration modules

These generated files are imported from the Rust crate through `wasm-bindgen` local JS modules and are copied into final application build output automatically. They are not meant to be served manually from a consumer project.

The shared runtime uses a small manual base module list in `build.mjs`. That base is chosen for the common/default extension set so optional or niche dependencies do not get baked into the bridge runtime unless they are broadly reused.

The TypeScript source for this package lives under

    src/
