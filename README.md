# leptos-tiptap

Use [Tiptap](https://tiptap.dev/) editors from [Leptos](https://leptos.dev/) applications.

The crate gives you two entry points:

- Use the `<TiptapEditor/>` component when you only need to mount an editor and drive it through the editor handle you
  pass in.
- Use the `use_tiptap_editor` hook when you want to provide the host element yourself.

## Usage

Add the crate to your Leptos app.

```toml
[dependencies]
leptos-tiptap = { version = "0.10", features = ["starter-kit"] }
```

The crate ships its precompiled Tiptap JavaScript and bridge sources as crate-local `wasm-bindgen` snippets. You do not
need to manually handle assets in any way.

## Cargo features

Tiptap is highly modularized. Nearly every aspect of it is a module (also named extension). Each module can be enabled
with a feature of this crate. Only the Tiptap code for enabled features will be shipped in your app!

Tiptap extensions are selected at compile time through Cargo features. Individual extensions can be enabled directly,
and the crate provides these common feature sets:

- The default features enable the `TiptapEditor` component and the minimal Tiptap schema: `document`, `paragraph`, and
  `text`.
- `starter-kit` also enables: blockquote, bold, bullet list, code, code block, document, dropcursor, gapcursor,
  hard break, heading, history, horizontal rule, italic, list item, ordered list, paragraph, strike, and text.
- `full` enables `starter-kit` plus: text alignment, highlight, image, link, placeholder, and YouTube support.

Enable `ssr` for server builds so JavaScript interop becomes a no-op while the editor host still renders for hydration.
The `nightly` feature enables Leptos' nightly APIs and forwards nightly support to `leptos-classes` and `leptos-styles`
when the `component` feature activates those optional dependencies.

## Component

Use the component when you are fine with the default editor host element.

```rust
use leptos::prelude::*;
use leptos_tiptap::{
    TiptapContent, TiptapEditor, TiptapEditorHandle,
    leptos_styles::Styles,
};

#[component]
pub fn EditorWithComponent() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (disabled, set_disabled) = signal(false);

    view! {
        <button on:click=move |_| set_disabled.update(|disabled| *disabled = !*disabled)>
            "Toggle disabled"
        </button>

        <button
            disabled=move || !handle.is_ready()
            on:click=move |_| {
                let _ = handle.toggle_bold();
            }
        >
            "Bold"
        </button>

        <TiptapEditor
            id="article-editor"
            handle=handle
            disabled=disabled
            initial_content=TiptapContent::html("<p>Edit me.</p>")
            on_change=move |_| {
                let _current_html = handle.get_html();
            }
            classes="editor"
            styles=Styles::builder()
                .with_unchecked("min-height", "12rem")
                .build()
        />
    }
}
```

The component populates the `handle` once the JavaScript editor is ready. Use that same handle to run commands,
read HTML or JSON content, or replace the full document with `set_content`, `set_html`, or `set_json`.

The component renders a `<div>` with the built-in `leptos-tiptap-instance` class. Its `classes` and `styles` props use
the `Classes` and `Styles` containers from the re-exported `leptos_classes` and `leptos_styles` modules, preserving
reactive values until they reach the host element.

Keep one handle per logical editor. You can retain it across that editor's conditional unmount/remount or a retry after
creation failure; a new session reports `NotReady` while it initializes. Do not share one handle between distinct or
concurrently mounted editors.

For advanced cases, `handle.instance()` returns the current `TiptapEditorInstance`. That value is bound to a concrete
mounted editor id and generation, so older instances become stale after destroy and recreate cycles.

## Hook

Use the hook when you want to choose the host element yourself or compose editor mounting into a larger component.

```rust
use leptos::prelude::*;
use leptos_tiptap::{use_tiptap_editor, TiptapContent, UseTiptapEditorInput};

#[component]
pub fn EditorWithHook() -> impl IntoView {
    let tiptap = use_tiptap_editor(UseTiptapEditorInput::new(
        "article-editor",
        TiptapContent::html("<p>Edit me.</p>"),
    ));

    let handle = tiptap.handle;
    let is_ready = tiptap.is_ready;
    let attrs = tiptap.props.into_attrs();

    view! {
        <button
            disabled=move || !is_ready.get()
            on:click=move |_| {
                let _ = handle.toggle_bold();
            }
        >
            "Bold"
        </button>

        <div {..attrs} class="editor"></div>
    }
}
```

Spread `tiptap.props.into_attrs()` onto exactly one rendered host element. The hook owns mount timing, cleanup,
disabled-state synchronization, and the reactive editor handle.

## Content, commands, and extensions

`initial_content` is one-time initialization input. Use `TiptapContent::html(...)` for HTML or
`TiptapContent::json(...)` / `TiptapContent::json_str(...)` for JSON. To replace content after mount, call
`handle.set_content(...)`, `handle.set_html(...)`, or `handle.set_json(...)`.

The editor `id` is a stable DOM id and must be unique across all live editor instances.

Extension-specific convenience commands such as `toggle_bold`, `set_link`, and `set_heading` focus the editor before
running the Tiptap command. Core position and selection commands are forwarded directly without that implicit focus
step.

Use `TiptapAttributes` for structured node and mark attributes. It supports insertion, lookup, borrowed map access,
consuming map access, and collection from key/value pairs.

Per-instance extension subsets can be selected with the component `extensions` prop or the hook input `extensions`
field; if omitted, all compiled extensions are active.

Selection callbacks receive an opaque `TiptapSelectionState`. Query boolean extension activity with typed keys:

```rust
use leptos_tiptap::TiptapActiveKey;

let bold_is_active = selection.is_active(TiptapActiveKey::Bold);
let bold_is_reported = selection.active(TiptapActiveKey::Bold); // Option<bool>
```

`active` distinguishes an explicitly inactive state from a key that the editor's selected extensions do not report.
Use `active_entries` to iterate every contributed boolean state. Separately typed selection information can be added to
the opaque aggregate without turning it into an untyped value map.

When the `placeholder` feature is enabled and active for an editor, set the component or hook `placeholder` option to
initialize its placeholder text. The extension adds placeholder classes and `data-placeholder`. Your app stylesheet must
render them, for example:

```css
.tiptap p.is-editor-empty:first-child::before,
.tiptap p.is-empty::before {
    color: #6b7280;
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
}
```

The official Tiptap Placeholder docs also include ready-to-copy CSS examples:
<https://tiptap.dev/docs/editor/extensions/functionality/placeholder>.

Bridge errors are reported through `on_error` as `TiptapEditorReport` values. Public editor operations return
`TiptapEditorResult<T>`, a `rootcause::Report` whose typed context is `TiptapEditorError`. Commands attempted while the
editor is not currently usable resolve to one of `TiptapEditorError::NotReady` (still mounting),
`TiptapEditorError::Destroyed` (cleanup ran), `TiptapEditorError::CreateFailed` (mount failed), or
`TiptapEditorError::Stale` (the `TiptapEditorInstance` refers to an editor that has since been destroyed and
recreated).

## Content sanitization

`leptos-tiptap` does not sanitize the content or attributes you pass it. HTML supplied to `TiptapContent::html` and to
`TiptapEditorHandle::set_html`, JSON supplied to `TiptapContent::json`/`set_json`, and resource attributes like
`TiptapLinkResource::href` and `TiptapYoutubeVideoResource::src` are forwarded to Tiptap as-is. Tiptap applies its own
schema-level filtering and the YouTube extension renders into a sandboxed iframe, but no scheme allowlist is enforced
here. If any of that input can come from an untrusted source, sanitize it on your side before handing it to the editor
(e.g. reject non-`http(s):` link hrefs, scrub HTML through a sanitizer such as `ammonia`).

## Integrated

If you are searching for a ready-to-use text editor, check out the leptos component library
[Leptonic](https://leptonic.dev/), which already incorporates this crate to define an editor.

## Development

### Repository layout

- The Rust crate lives at the repository root (`Cargo.toml`, `src/`, `tests/`).
- The browser-side Tiptap host lives in `tiptap/` as a TypeScript build project. `npm run build` (driven by
  `just build`) produces one bundle per Tiptap extension in `src/js/generated/`. Those bundles are checked in and
  shipped with the crate as crate-local `wasm-bindgen` snippets.
- Example apps live in `examples/`. `demo-app` is the shared UI. `demo-csr` and `demo-ssr` are thin CSR/SSR wrappers
  around it. `demo-app` also hosts fixtures used for browser-based integration testing.

### Library prerequisites

- A Rust toolchain matching the crate's MSRV, currently `1.89.0`, or newer.
- The `wasm32-unknown-unknown` target for Leptos app development
  (`rustup target add wasm32-unknown-unknown`).

Library users do not need Node.js; the generated Tiptap JavaScript is included in the crate.

### Development prerequisites

In addition to the library prerequisites above, contributors need:

- A nightly toolchain with `wasm32-unknown-unknown` installed when running `just verify-nightly`.
- Node.js 24 or newer with `npm` to rebuild and validate the Tiptap JavaScript bundle.
- [`just`](https://github.com/casey/just) for the orchestration recipes.

### Common commands

Run the examples:

```sh
cd examples/demo-ssr && cargo leptos watch
cd examples/demo-csr && trunk serve
```

Run `just` to list all available recipes.

- `just build` performs a reproducible rebuild from `tiptap/package-lock.json`.
- `just update-tiptap` resolves and pins all official packages to the newest stable Tiptap 2 release, refreshes
  transitive npm dependencies, and rebuilds the checked-in bundle artifacts. Pass a specific stable 2.x version when
  needed, for example `just update-tiptap 2.27.1`.
- `just verify` runs the full Rust and bridge-level validation suite, including formatting and a generated-bundle
  drift check.

## Leptos compatibility

| Crate version | Compatible Leptos version |
|---------------|---------------------------|
| 0.9, 0.10     | 0.8                       |
| 0.8           | 0.7                       |
| 0.7           | 0.6                       |
| 0.5, 0.6      | 0.5 (csr and ssr)         |
| 0.4           | 0.5 (csr)                 |
| 0.3.0-rc1     | 0.5.0-rc1                 |
| 0.3.0-beta    | 0.5.0-beta                |
| 0.3.0-alpha   | 0.5.0-alpha               |
| 0.2           | 0.4                       |
| 0.1           | 0.3                       |

## MSRV

The minimum supported Rust version is `1.89.0`.

## Contributing

Contributions are very welcome. We appreciate bug reports, feature ideas, documentation improvements, and pull
requests of all sizes.
