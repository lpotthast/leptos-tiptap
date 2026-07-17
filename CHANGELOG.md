# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [leptos-tiptap 0.10.0] - 2026-07-17

### Important: JavaScript delivery and deployment changed

This release replaces the external `/js/tiptap.js` asset contract with crate-local `wasm-bindgen` snippets. The
precompiled Tiptap bridge and extension modules are now emitted into the application's generated JavaScript/Wasm output
as part of its normal build. Consumer applications therefore no longer need `leptos-tiptap-build`, a downstream
`build.rs`, a copied `tiptap.js`, or a manual preload tag.

Applications with custom asset pipelines or server configuration must account for the new output layout:

- Publish and serve the complete generated JavaScript/Wasm output recursively, including its `snippets/` subtree, with
  the correct JavaScript MIME type. The crate arranges for these files to be emitted; the application server or CDN is
  still responsible for serving them.
- Replace CDN, reverse-proxy, cache, CSP, CORS, service-worker, and precache rules that targeted `/js/tiptap.js`. That
  URL is no longer used by the runtime.
- Deploy the generated JavaScript glue, Wasm, and snippet files together. Treat generated snippet paths as internal
  build output rather than stable public URLs, and ensure cached files are invalidated when replacing a build.

### Added

- Added the `use_tiptap_editor` hook for mounting an editor on a custom host element. `UseTiptapEditorInput::new` and
  `Default` provide concise configuration for the common case.
- Added per-editor extension selection through `TiptapExtension` and the `extensions` prop, plus placeholder support
  through the `placeholder` feature and prop.
- Added a `nightly` feature that enables `leptos/nightly` and forwards nightly support to the optional
  `leptos-classes` and `leptos-styles` component dependencies when they are active.

### Changed

- Replaced `<TiptapInstance/>`, `TiptapInstanceMsg`, and the `msg`, `value`, and `set_value` props with
  `<TiptapEditor/>`, its `handle: TiptapEditorHandle` prop, one-time `initial_content`, and readiness, change, and error
  callbacks. The handle now sends commands and reads or replaces the live document as HTML or JSON. Advanced callers
  can access the generation-bound `TiptapEditorInstance`.
- Changed commands and document operations to return `TiptapEditorResult` with typed `TiptapEditorError` causes.
  Asynchronous bridge failures are reported through `on_error`.
- Changed editor ids from reactive reset inputs to stable DOM ids that must be unique among live instances.
- Changed `<TiptapEditor/>` to render a `<div class="leptos-tiptap-instance">` host and accept reactive `classes` and
  `styles` props through the re-exported `leptos_classes` and `leptos_styles` modules.
- Changed extension compilation to use individual Cargo features, with `starter-kit` and `full` presets. The
  `text_align` feature now enables its required `heading` and `paragraph` schema features.
- Changed `TiptapSelectionState` from feature-gated public boolean fields to an opaque aggregate with typed,
  feature-independent `TiptapActiveKey` lookups through `active`, `is_active`, and `active_entries`.
- Changed `TiptapContent::Json` from a string to `serde_json::Value`. Use `TiptapContent::json_str` to parse serialized
  JSON.
- Updated extension resource types: `TiptapImageResource` uses `src` and optional `alt`/`title`, `TiptapLinkResource`
  uses optional `target`/`rel`/`class`, and `TiptapYoutubeVideoResource` uses optional integer
  `start`/`width`/`height` values.
- Raised the `leptos-tiptap` MSRV to `1.89.0`.
- Updated the bundled TipTap packages from `2.12.0` to `2.27.2`.

### Removed

- Removed the legacy `leptos-tiptap-build` crate and the downstream `build.rs`, copied asset, and preload-tag setup it
  required. The runtime crate now ships the JavaScript bridge itself; see the deployment migration notice above.
- Removed the public `TiptapEditorState` type.

## [leptos-tiptap-build 0.2.9] - 2026-04-08

### Added

- Added a legacy compatibility notice directing new integrations to the runtime crate's built-in JavaScript bundle.

## [leptos-tiptap-build 0.2.8] - 2025-06-01

### Changed

- Updated the bundled TipTap packages from `2.10.4` to `2.12.0`.

## [leptos-tiptap 0.9.0] - 2025-06-01

### Changed

- Migrated the runtime crate and examples to Leptos `0.8`.

## [leptos-tiptap-build 0.2.7] - 2024-12-28

### Added

- Added bundled support for links, embedded YouTube videos, and ordered and unordered lists.

### Changed

- Updated the bundled TipTap packages from `2.2.0` to `2.10.4`.

## [leptos-tiptap 0.8.0] - 2024-12-28

### Added

- Added editing commands for links, embedded YouTube videos, and ordered and unordered lists.

### Changed

- Migrated the runtime crate to Leptos `0.7`.
- Raised the MSRV to `1.76.0`.

### Fixed

- Fixed editor teardown checks that could fail on route changes.
- Fixed callback conversion failures after the Leptos `0.7` migration.

## [leptos-tiptap 0.7.0] - 2024-02-01

### Changed

- Migrated the runtime crate and examples to Leptos `0.6`.

## [leptos-tiptap-build 0.2.5] - 2024-01-30

### Changed

- Updated the bundled TipTap packages from `2.1.12` to `2.2.0`.

## [leptos-tiptap 0.6.0] - 2024-01-30

### Changed

- Allowed reactive editor `id` changes so consumers could recreate editor instances by changing the id.

### Fixed

- Fixed a runtime panic during editor cleanup.

## [leptos-tiptap 0.5.0] - 2024-01-30

### Added

- Added server-side rendering compatibility through the `ssr` feature.

### Changed

- Switched the `wasm_bindgen` raw module path to the absolute `/js/tiptap.js` path expected by consuming apps.

## [leptos-tiptap-build 0.2.4] - 2023-10-18

### Changed

- Updated the bundled TipTap packages from `2.1.8` to `2.1.12`.

## [leptos-tiptap 0.4.0] - 2023-10-18

### Changed

- Migrated the runtime crate to Leptos `0.5.1`.

## [leptos-tiptap-build 0.2.3] - 2023-09-13

### Changed

- Updated the bundled TipTap packages from `2.1.7` to `2.1.8`.

## [leptos-tiptap 0.3.0-rc1] - 2023-09-13

### Changed

- Migrated the runtime crate to the Leptos `0.5.0-rc1` line.

## [leptos-tiptap-build 0.2.2] - 2023-09-04

### Changed

- Updated the bundled TipTap packages from `2.0.3` to `2.1.7`.
- Corrected the build crate's declared MSRV.

## [leptos-tiptap 0.3.0-beta] - 2023-09-04

### Changed

- Migrated the runtime crate to the Leptos `0.5.0-beta` line.

## [leptos-tiptap 0.3.0-alpha] - 2023-07-28

### Changed

- Migrated the runtime crate to Leptos `0.5.0-alpha`.

### Fixed

- Fixed disabled state propagation between the Leptos component and the JS editor instance.

## [leptos-tiptap 0.2.0] - 2023-07-01

### Added

- Added the `leptos-tiptap-build` helper crate for shipping the generated JavaScript assets.

### Changed

- Migrated the runtime crate from Leptos `0.3` to Leptos `0.4`.
- Simplified downstream `build.rs` integration and refreshed the generated TipTap bundle.

### Fixed

- Fixed selection-state reporting so updates are emitted immediately and after editor actions.
- Fixed DOM/accessibility handling by rendering a custom element instead of a plain `div` and forwarding
  `aria-disabled`.

## [leptos-tiptap-build 0.1.1] - 2023-07-01

### Changed

- Refreshed the generated TipTap bundle and simplified downstream build integration.

## [leptos-tiptap-build 0.1.0] - 2023-06-25

### Added

- Added the initial `leptos-tiptap-build` crate for embedding generated TipTap JavaScript assets.

## [leptos-tiptap 0.1.0] - 2023-06-13

### Added

- Added the initial Leptos/Tiptap integration with the `TiptapInstance` component, browser bridge, and content and
  selection-state APIs.
