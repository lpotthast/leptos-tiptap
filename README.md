# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

Current repository versions:

- `leptos-tiptap`: `0.10.0`
- `leptos-tiptap-build`: `0.2.8`
- `leptos`: `0.8.2`
- Tiptap npm packages in `tiptap/package.json`: `2.27.2`

Currently bundled Tiptap packages:

- "@tiptap/core": "^2.27.2",
- "@tiptap/extension-highlight": "^2.27.2",
- "@tiptap/extension-image": "^2.27.2",
- "@tiptap/extension-youtube": "^2.27.2",
- "@tiptap/extension-link": "^2.27.2",
- "@tiptap/extension-text-align": "^2.27.2",
- "@tiptap/starter-kit": "^2.27.2"

This repository contains:

| Dir                 | What is it for?                                                                                                                                                                                                                              |
|---------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| leptos-tiptap       | The main dependency other leptos projects can depend on. It provides the `<TiptapInstance>` component through which a tiptap instance is managed automatically.                                                                              |
| leptos-tiptap-build | This dependency can be used in `build.rs` scripts. It provides the generated Tiptap browser asset. `TIPTAP_JS` is the bundled `/js/tiptap.js` browser module containing Tiptap, the bundled extensions, and this project's JS adapter layer. |
| tiptap              | Build process for the generated Tiptap browser assets. `just build` bundles the adapter with `esbuild`, writes the outputs into `leptos-tiptap-build/dist/`, and keeps the example apps in sync.                                             |

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

`just update-tiptap` is the explicit maintenance command for upgrading the npm dependencies and refreshing the checked-in bundle artifacts.

## Runtime notes

- `TiptapInstance` takes `initial_content: TiptapContent` and applies it once when the editor is created. Use `TiptapContent::Html(...)` or `TiptapContent::Json(...)` to choose the initialization format.
- `TiptapInstance` is not a controlled component. The editor keeps its own internal state after creation.
- `TiptapInstance` notifies Rust through `on_ready` and `on_change`. Both callbacks receive a `TiptapEditorHandle`, which can be stored for the lifetime of that editor instance and can read the current editor content as HTML or JSON on demand.
- `TiptapInstance` can also report bridge/runtime failures through `on_error`.
- `TiptapEditorHandle` can also replace the full document content explicitly through `set_content`, `set_html`, or `set_json`.
- If an editor is destroyed and recreated with the same DOM id, older handles become stale and fail with `EditorUnavailable` instead of targeting the replacement editor.
- A single editor instance can be read back in multiple formats. The current HTML/JSON distinction is no longer tied to the initial content format.
- The `id` prop is a stable DOM id for the editor root and must stay unique across all live editor instances.
- `/js/tiptap.js` is the authoritative JS module imported by the Rust bridge. It is the bundled browser module containing Tiptap, the bundled extensions, and this project's JS adapter layer.
