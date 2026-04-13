# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

Current repository versions:

- `leptos-tiptap`: `0.10.0`
- `leptos`: `0.8.2`
- Tiptap npm packages in `tiptap/package.json`: `2.27.2`

Current default crate feature set:

- component, document, paragraph, text

Optional feature bundles:

- `starter-kit`: blockquote, bold, bullet list, code, code block, document, dropcursor, gapcursor,
  hard break, heading, history, horizontal rule, italic, list item, ordered list, paragraph,
  strike, text
- `full`: `starter-kit` plus text-align, highlight, image, link, placeholder, youtube

This repository contains:

| Dir           | What is it for?                                                                                                                                                                                 |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| leptos-tiptap | The main dependency other leptos projects can depend on. It provides the `<TiptapEditor>` component through which a tiptap instance is managed automatically.                                   |
| tiptap        | Build process for the crate-local JS snippets. `just build` bundles the shared bridge runtime and standalone extension modules into `leptos-tiptap/src/js/generated/`. |

## Run the examples

- demo-csr (requires `cargo install trunk`)

```sh
    cd leptos-tiptap/examples/demo-csr && trunk serve
```

- demo-ssr (requires `cargo install cargo-leptos`)

```sh
    cd leptos-tiptap/examples/demo-ssr && cargo leptos watch
```

## Build tiptap JS

`just build` performs a reproducible rebuild from `tiptap/package-lock.json`.

```sh
just build
```

`just update-tiptap` is the explicit maintenance command for upgrading the npm dependencies and refreshing the
checked-in bundle artifacts.

`just verify` runs the Rust and bridge-level validation suite, including a generated-bundle drift check.

## Runtime notes

- Consumers only depend on `leptos-tiptap`. They do not need a `build.rs`, copied `tiptap.js`, or a manual
  `<link rel="modulepreload">`.
- The JS bridge is shipped as crate-local `wasm-bindgen` snippets. Those files end up in the final app build output
  automatically.
- `<TiptapEditor/>` takes `initial_content: TiptapContent` and applies it once when the editor is created. Use
  `TiptapContent::Html(...)` or `TiptapContent::Json(...)` to choose the initialization format.
- Cargo features control which extensions are compiled into the runtime. The `extensions` prop on `<TiptapEditor/>`
  controls which of those compiled extensions are active for a specific editor instance.
- If `extensions` is omitted, `<TiptapEditor/>` activates all extensions compiled into the current build.
- The `placeholder` feature and `<TiptapEditor placeholder=.../>` configure Tiptap's placeholder extension, but visible
  placeholder text still requires app CSS for the generated empty-node classes and `data-placeholder` attribute. See
  the official Tiptap Placeholder docs for copy-paste CSS:
  <https://tiptap.dev/docs/editor/extensions/functionality/placeholder>.
- Per-instance extension subsets are validated before mount. If an extension needs another extension, such as
  list support needing `list_item`, the editor reports a clear runtime error instead of passing an invalid schema to
  Tiptap.
- Extension-specific Rust command methods are only available when the corresponding Cargo feature is enabled.
- Enabling the `text_align` Cargo feature also enables the required `heading` and `paragraph` schema nodes so
  alignment commands can succeed consistently.
- `<TiptapEditor/>` is not a controlled component. The editor keeps its own internal state after creation.
- `<TiptapEditor/>` notifies Rust through `on_ready`, `on_change`, and `on_selection_change`. `on_ready` runs before
  the initial selection-state notification, so the `editor` prop is ready when selection callbacks run.
- `<TiptapEditor/>` can also report bridge/runtime failures through `on_error`.
- `TiptapEditorHandle` is the user-held reactive handle for commands and content reads. `TiptapEditorInstance` remains
  available through `TiptapEditorHandle::instance()` for advanced use. `instance()` is a tracked reactive read; use
  `instance_untracked()` in event handlers when you do not want to subscribe to readiness changes.
- If an editor is destroyed and recreated with the same DOM id, older live instances become stale and fail with
  `EditorUnavailable` instead of targeting the replacement editor. That stale-instance path is treated as expected
  control flow and no longer logs a JS runtime error.
- Mounting two live editors with the same DOM id is rejected instead of replacing the existing editor.
- A single editor instance can be read back in multiple formats. The current HTML/JSON distinction is no longer tied to
  the initial content format.
- Low-level list-item split commands accept structured `TiptapAttributes` maps for override attributes.
- The `id` prop is a stable DOM id for the editor root and must stay unique across all live editor instances.
- The internal JS packaging uses one shared bridge runtime plus standalone official extension modules. The bridge runtime carries the common Tiptap/ProseMirror base used by the default extension set, while niche extension-specific dependencies stay local to their extension bundles.
