# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

## Usage

This is a rather low-level dependency. Use it if you want to create your own editor experience.
You will need the generated Tiptap JS assets in your application.
Check out `leptos-tiptap-build` as well as the `leptos-tiptap` examples ("demo-csr" and "demo-ssr"), all available in the [repository](https://github.com/lpotthast/leptos-tiptap),
to get a grasp of how the JS files can be added to a build.

`TiptapInstance` takes `initial_content: TiptapContent` and uses it once when the editor is created.
Use `TiptapContent::Html(...)` for HTML input or `TiptapContent::Json(serde_json::Value)` for structured TipTap JSON input.

When the editor becomes ready or changes, `TiptapInstance` gives you a `TiptapEditorHandle`.
Use that handle to pull the current content as HTML or JSON from the same live editor instance,
or to replace the full document content through `set_content`, `set_html`, or `set_json`.
It is safe to store that handle for the lifetime of the current editor instance.
If the editor is later destroyed and recreated with the same DOM id, the old handle becomes stale
and returns `EditorUnavailable` instead of addressing the replacement editor.

If the JS bridge can not create the editor, parse content, or apply a command, `TiptapInstance`
reports that through the optional `on_error` callback.

The `id` prop is a stable DOM id for the editor root and must be unique across all live editor instances.

`/js/tiptap.js` is the browser module imported by the Rust bridge. It contains the bundled Tiptap runtime, the bundled extensions, and this project's JS adapter layer.

## Integrated

If you are searching for a ready-to-use text editor, check out the leptos component
library [Leptonic](https://leptonic.dev/), which already incorporates this crate to define an editor.

## Leptos compatibility

| Crate version | Compatible Leptos version |
|---------------|---------------------------|
| 0.1           | 0.3                       |
| 0.2           | 0.4                       |
| 0.3.0-alpha   | 0.5.0-alpha               |
| 0.3.0-beta    | 0.5.0-beta                |
| 0.3.0-rc1     | 0.5.0-rc1                 |
| 0.4,          | 0.5 (csr)                 |
| 0.5, 0.6      | 0.5 (csr and ssr)         |
| 0.7           | 0.6                       |
| 0.8           | 0.7                       |
| 0.9, 0.10     | 0.8                       |

## MSRV

The minimum supported rust version is `1.89.0`.
