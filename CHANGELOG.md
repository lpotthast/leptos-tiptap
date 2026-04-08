# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [leptos-tiptap 0.10.0] - Unreleased

### Added

- Added command methods to `TiptapEditorHandle` (`toggle_bold`, `toggle_italic`, `toggle_heading`, `set_paragraph`, `set_link`, etc.) so callers interact with the editor directly through the handle instead of a message signal.
- Added `TiptapEditorHandle`, which can read the current editor content as HTML or JSON from the same live editor instance.
- Added `TiptapEditorHandle::set_content`, `set_html`, and `set_json` for explicit full-document replacement on a live editor instance.
- Added `on_ready` and `on_change` callbacks on `TiptapInstance` so callers can pull the current content on demand.
- Added an optional `on_error` callback on `TiptapInstance` for JS bridge and runtime failures.
- Added browser-level SSR coverage in the `demo-ssr` example with Playwright, including hydration and HTML/JSON round-trip checks.
- Added adapter-level JS unit tests covering create failures, invalid content handling, and editor registry cleanup.
- Added adapter-level tests covering generation-aware stale-handle rejection and the initial selection callback emitted during editor startup.
- Added zero-config crate-local JS snippet delivery for the bundled browser runtime, so `leptos-tiptap` can ship its generated Tiptap bridge directly through `wasm-bindgen`.
- Added the public `TiptapExtension` selection type and an `extensions` prop on `TiptapInstance` for explicit per-editor extension activation.
- Added this root `CHANGELOG.md`.

### Changed

- Changed `TiptapInstance` to use `TiptapEditorHandle` command methods instead of the `msg: Signal<TiptapInstanceMsg>` prop. The `msg` prop has been removed.
- Changed the editor readiness tracking to use a reactive signal internally, so the disabled state is automatically synced when the editor becomes ready without a manual call from the `on_ready` closure.
- Changed the internal `TiptapEditorHandle` identity model to include a JS-side generation token so stale handles can no longer address a recreated editor instance that reused the same DOM id.
- Changed `TiptapInstance` to take `initial_content: TiptapContent` instead of `value: Signal<TiptapContent>`.
- Changed the content callback model from eager serialized payload pushes to lightweight change notifications plus explicit HTML/JSON readback through `TiptapEditorHandle`.
- Changed the runtime model so one editor instance can be read back as both HTML and JSON, regardless of which format was used for initial content.
- Changed `TiptapInstance` so `id` is now a stable DOM id instead of a reactive reset mechanism.
- Changed the JS bundle workflow to use `npm ci` for reproducible rebuilds and a separate `just update-tiptap` maintenance command for dependency upgrades.
- Changed the JS packaging from the old external `/js/tiptap.js` asset contract to crate-local `wasm-bindgen` snippets. Consumer apps now depend only on `leptos-tiptap` and no longer need a downstream `build.rs`, copied browser assets, or a manual preload tag.
- Changed the internal JS architecture from one monolithic adapter bundle to a bridge runtime, a separated Tiptap core runtime, and standalone official extension registration modules.
- Changed the runtime preset to use explicit extension modules instead of `StarterKit` as the shipped integration unit.
- Changed extension compilation and activation to be explicit: Cargo features decide which extensions are compiled and registered, and each editor instance now activates a chosen subset or defaults to all compiled extensions.
- Changed the example applications and README guidance to use the new zero-config integration path.
- Changed the CSR and SSR demos to show one editor with side-by-side HTML and JSON readbacks instead of treating them as separate editor modes.
- Changed unit tests to use the `assertr` crate for assertions.
- Changed bridge command handling to return structured statuses instead of booleans or missing values.
- Changed the link and YouTube resource payloads to use optional metadata fields instead of stringly-typed required values.
- Changed the SSR Playwright test to reuse a single expected editor id constant instead of repeating the selector/id literal.
- Raised the `leptos-tiptap` MSRV to `1.89.0` to match the adopted `assertr` version.
- Updated the demos and README files to reflect the new API, current versions, and stable editor ids.
- Updated the bundled TipTap packages from `2.12.0` to `2.27.2`.

### Fixed

- Fixed leaked editor ids in the global JS registry by deleting entries instead of storing `undefined`.
- Fixed the mismatch between the public `TiptapContent` API and the actual runtime behavior by separating initial content input from content readback.
- Fixed JS command handling to log clear errors when Rust sends commands before an editor instance exists.
- Fixed editor lifecycle tracking so a JS create failure does not leave Rust thinking an editor is active.
- Fixed a synchronous create/ready race between the Rust component and JS adapter so startup callbacks no longer leave the component in an inconsistent lifecycle state.
- Fixed `set_content` error reporting so invalid content is no longer collapsed into `EditorUnavailable`.
- Fixed stale `TiptapEditorHandle` instances so they now fail with `EditorUnavailable` instead of accidentally targeting a newer editor registered under the same id.
- Fixed invalid selection payload handling so deserialization failures are reported without emitting a fake default selection state.
- Fixed the SSR demo so it no longer implies that separate editors are required to inspect HTML and JSON representations of the same document.
- Fixed stale and inconsistent version/build documentation in the repository README files.
- Fixed repository hygiene so example `public/js` bundle copies are treated as generated artifacts instead of tracked source files.
- Fixed the demos so they build directly against crate-local snippets without the old asset-copy setup.

### Removed

- Removed the legacy `leptos-tiptap-build` crate from this repository after publishing its final compatibility release.
- Removed `TiptapInstanceMsg` and the `msg` signal prop from `TiptapInstance`. Use `TiptapEditorHandle` command methods instead.
- Removed the internal `MessageBufferState` pre-ready command buffering. Commands are now sent directly through the handle after `on_ready` fires.
- Removed the unusable public `TiptapEditorState` type from the `leptos-tiptap` API surface.
- Removed dead JS bridge declarations that only existed for the removed editor-state surface.
- Removed the `set_value: Callback<(TiptapContent,)>` update path from `TiptapInstance`.
- Removed the format-bound callback model where a live editor instance always pushed updates in the same representation it was created with.
- Removed the example `build.rs` scripts, copied `public/js/tiptap.js` assets, and manual module-preload tags from the supported setup path.

## [leptos-tiptap-build 0.2.8] - 2025-06-01

### Changed

- Updated the bundled TipTap packages from `2.10.4` to `2.27.2`.

## [leptos-tiptap 0.9.0] - 2025-06-01

### Changed

- Migrated the runtime crate and examples to Leptos `0.8`.
- Updated the wasm-bindgen dependency line used by the runtime crate.

## [leptos-tiptap-build 0.2.7] - 2024-12-28

### Added

- Added bundled support for links, embedded YouTube videos, and ordered and unordered lists.

### Changed

- Updated the bundled TipTap packages from `2.2.0` to `2.10.4`.

## [leptos-tiptap-build 0.2.6] - 2024-02-01

### Removed

- Removed an unused dependency from the build crate.

## [leptos-tiptap 0.8.0] - 2024-12-28

### Added

- Added `TiptapInstanceMsg` variants and runtime wiring for links and embedded YouTube videos.
- Added `BulletList` and `OrderedList` commands plus demo support for list editing.
- Added `send_wrapper` to support the callback handling used by the Leptos `0.7` migration.
- Added README files for the CSR and SSR demo applications.

### Changed

- Migrated the runtime crate and examples to Leptos `0.7`.
- Refreshed the CSR demo implementation and SSR demo app structure to match the new Leptos APIs.
- Raised the MSRV to `1.76.0`.

### Fixed

- Fixed editor teardown checks that could fail on route changes.
- Fixed callback conversion issues by using tuple-typed `Callback` signatures.
- Fixed small findings and clippy issues discovered during the migration work.

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

- Fixed cleanup behavior by switching the lifecycle cleanup path to non-tracking access.
- Fixed a runtime panic in the editor lifecycle code.

## [leptos-tiptap 0.5.0] - 2024-01-30

### Added

- Added SSR compatibility to the runtime crate.
- Added the dedicated `demo-ssr` example application.
- Renamed the original example app to `demo-csr`.
- Added the `ssr` feature flag to the runtime crate.

### Changed

- Switched the `wasm_bindgen` raw module path to the absolute `/js/tiptap.js` path expected by consuming apps.

## [leptos-tiptap-build 0.2.4] - 2023-10-18

### Changed

- Updated the bundled TipTap packages from `2.1.8` to `2.1.12`.
- Standardized README casing to `README.md`.

## [leptos-tiptap 0.4.0] - 2023-10-18

### Changed

- Migrated the runtime crate to Leptos `0.5.1`.
- Updated serde-related dependency versions used by the runtime crate.
- Standardized the crate README filename to `README.md`.

## [leptos-tiptap-build 0.2.3] - 2023-09-13

### Changed

- Updated the bundled TipTap packages from `2.1.7` to `2.1.8`.
- Refreshed the build-crate documentation.

## [leptos-tiptap 0.3.0-rc1] - 2023-09-13

### Changed

- Migrated the runtime crate to the Leptos `0.5.0-rc1` line.
- Refreshed the accompanying runtime and build-crate documentation.

## [leptos-tiptap-build 0.2.2] - 2023-09-04

### Changed

- Updated the bundled TipTap packages from `2.0.3` to `2.1.7`.
- Corrected the build-crate MSRV and normalized explicit version declarations in the crate metadata.

## [leptos-tiptap 0.3.0-beta] - 2023-09-04

### Changed

- Migrated the runtime crate to the Leptos `0.5.0-beta` line.

## [leptos-tiptap 0.3.0-alpha] - 2023-07-28

### Added

- Added MSRV documentation and normalized explicit version declarations in the crate metadata.
- Added clearer documentation around disabled-state behavior and simplified the example application.

### Changed

- Migrated the runtime crate to Leptos `0.5.0-alpha`.
- Refreshed crate metadata and documentation during the Leptos `0.5.0-alpha` migration.

### Fixed

- Fixed disabled state propagation between the Leptos component and the JS editor instance.

## [leptos-tiptap-build 0.2.0] - 2023-07-02

### Changed

- Started the `0.2.x` build-crate line used to embed the generated TipTap browser assets for downstream `build.rs` scripts.

## [leptos-tiptap 0.2.0] - 2023-07-01

### Added

- Added the `leptos-tiptap-build` helper crate and documented how to ship the generated JS assets.
- Added crate metadata, licenses, and README files.
- Added a Leptos compatibility table to the documentation.

### Changed

- Migrated the runtime crate from Leptos `0.3` to Leptos `0.4`.
- Simplified the example `build.rs` setup and refreshed the generated TipTap bundle.

### Fixed

- Fixed selection-state reporting so updates are emitted immediately and after editor actions.
- Fixed DOM/accessibility handling by rendering a custom element instead of a plain `div` and forwarding `aria-disabled`.
- Removed unused code from compilation and removed an unused runtime dependency.

## [leptos-tiptap-build 0.1.1] - 2023-07-01

### Changed

- Bumped the build crate to `0.1.1` alongside the refreshed generated TipTap bundle and simplified example build integration.

## [leptos-tiptap 0.1.1] - 2023-06-25

### Added

- Added project metadata, licenses, and README files.
- Added and documented the stable Leptos feature configuration used by the crate at the time.

### Changed

- Bumped the crate to `0.1.1`.
- Removed the committed `Cargo.lock` from the crate workspace.

### Fixed

- Removed unused code from the compiled crate.

## [leptos-tiptap-build 0.1.0] - 2023-06-25

### Added

- Added the initial `leptos-tiptap-build` crate for embedding the generated TipTap JavaScript assets.
- Added build-crate documentation and committed distribution assets needed by downstream examples.

## [leptos-tiptap 0.1.0] - 2023-06-13

### Added

- Added the initial Leptos/Tiptap integration crate.
- Added the `TiptapInstance` component and the initial JS bridge to the browser-side TipTap editor.
- Added the initial content and selection-state types used by the runtime API.

### Changed

- Iterated on the public type layout and dependency organization during the initial implementation.

### Fixed

- Ensured selection-state updates are emitted after editor actions.
- Rendered the editor host as a custom element and forwarded `aria-disabled` state.
