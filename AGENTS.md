# Repository Guidelines

## Project Structure & Module Organization

This repository wraps Tiptap for Leptos. The main Rust library crate lives in `leptos-tiptap/`; public API types are in
`leptos-tiptap/src/lib.rs`, component-facing API is under `leptos-tiptap/src/api/`, and runtime bridge/session code is
under `leptos-tiptap/src/runtime/`.

The browser-side Tiptap bundle source lives in `tiptap/`. TypeScript bridge code is in `tiptap/src/`, extension entry
points use `tiptap/src/extensions/tiptap_*.ts`, and generated JS consumed by Rust is written to
`leptos-tiptap/src/js/generated/`. Example apps are under `leptos-tiptap/examples/`, including `demo-csr` and
`demo-ssr`.

## Build, Test, and Development Commands

- `just build`: install pinned npm dependencies and rebuild the generated Tiptap JS bundle.
- `just verify`: run JS tests, TypeScript checks, generated-bundle drift checks, Rust tests, SSR/wasm builds, and
  Clippy.
- `cd leptos-tiptap && cargo build`: build the library crate.
- `cd leptos-tiptap && cargo build --features ssr`: verify the server-side no-op JS path builds.
- `cd tiptap && npm test`: run bridge runtime tests.
- `cd leptos-tiptap/examples/demo-csr && trunk serve`: run the CSR example.
- `cd leptos-tiptap/examples/demo-ssr && cargo leptos watch`: run the SSR example.

## Coding Style & Naming Conventions

Use Rust 2024 formatting via `cargo fmt`; keep public Rust types and functions idiomatic with `CamelCase` types and
`snake_case` modules/functions. Use TypeScript ES modules in `tiptap/`, and name extension modules
`tiptap_<extension>.ts`. When adding or removing commands, update Rust protocol/API/runtime code and the matching
TypeScript bridge or extension module together.

## Testing Guidelines

Place Rust tests near the code they cover or in crate-level test modules, and use the `assertr` crate for assertions
instead of `assert!` or `assert_eq!`. For Tiptap bridge behavior, add Node tests in `tiptap/src/bridge_runtime.test.ts`.
Run targeted checks during development, then run `just verify` before submitting broad changes.

## Commit & Pull Request Guidelines

Recent history uses short, imperative commit subjects such as `Add end2end tests` and `Update tiptap to 2.12.0`. Keep
commits focused and mention generated bundle updates when `leptos-tiptap/src/js/generated/` changes. Pull requests
should describe the behavior change, list verification commands run, link related issues, and include screenshots or
reproduction notes for example-app UI changes.

## Runtime Notes

`TiptapEditor` treats `initial_content` as one-time initialization input; use `TiptapEditorHandle` for later content
reads and document replacement. Editor `id` values must be globally unique among live instances. With the `ssr` feature
enabled, JS interop in `leptos-tiptap/src/runtime/` is compiled as no-op behavior while still rendering the DOM node for
hydration.
