# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

Current repository versions:

- `leptos-tiptap`: `0.10.0`
- `leptos`: `0.8.2`
- Tiptap npm packages in `tiptap/package.json`: `2.27.2`

Current default compiled extension feature set:

- blockquote, bold, bullet list, code, code block, document, dropcursor, gapcursor
- hard break, heading, history, horizontal rule, italic, list item, ordered list, paragraph
- strike, text, text-align, highlight, image, link, youtube

This repository contains:

| Dir           | What is it for?                                                                                                                                                                                 |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| leptos-tiptap | The main dependency other leptos projects can depend on. It provides the `<TiptapInstance>` component through which a tiptap instance is managed automatically.                                 |
| tiptap        | Build process for the crate-local JS snippets. `just build` bundles the bridge runtime, separated Tiptap core runtime, and standalone extension modules into `leptos-tiptap/src/js/generated/`. |

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

## Runtime notes

- Consumers only depend on `leptos-tiptap`. They do not need a `build.rs`, copied `tiptap.js`, or a manual
  `<link rel="modulepreload">`.
- The JS bridge is shipped as crate-local `wasm-bindgen` snippets. Those files end up in the final app build output
  automatically.
- `TiptapInstance` takes `initial_content: TiptapContent` and applies it once when the editor is created. Use
  `TiptapContent::Html(...)` or `TiptapContent::Json(...)` to choose the initialization format.
- Cargo features control which extensions are compiled into the runtime. The `extensions` prop on `TiptapInstance`
  controls which of those compiled extensions are active for a specific editor instance.
- If `extensions` is omitted, `TiptapInstance` activates all extensions compiled into the current build.
- `TiptapInstance` is not a controlled component. The editor keeps its own internal state after creation.
- `TiptapInstance` notifies Rust through `on_ready` and `on_change`. Both callbacks receive a `TiptapEditorHandle`,
  which can be stored for the lifetime of that editor instance and can read the current editor content as HTML or JSON
  on demand.
- `TiptapInstance` can also report bridge/runtime failures through `on_error`.
- `TiptapEditorHandle` can also replace the full document content explicitly through `set_content`, `set_html`, or
  `set_json`.
- If an editor is destroyed and recreated with the same DOM id, older handles become stale and fail with
  `EditorUnavailable` instead of targeting the replacement editor.
- A single editor instance can be read back in multiple formats. The current HTML/JSON distinction is no longer tied to
  the initial content format.
- The `id` prop is a stable DOM id for the editor root and must stay unique across all live editor instances.
- The internal JS packaging is now split into one bridge runtime, one separated Tiptap core runtime, and standalone
  official extension modules. That split is internal for now; the default editor behavior remains unchanged.
