# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

## Usage

This is a rather low-level dependency. Use it if you want to create your own editor experience.
You will need the actual tiptap JS code in your application.
Check out `leptos-tiptap-build` as well as the `leptos-tiptap` examples ("demo-csr" and "demo-ssr"), all available in the [repository](https://github.com/lpotthast/leptos-tiptap),
to get a grasp of how the tiptap JS files can be added to a build.

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
| 0.4           | 0.5 (csr)                 |
| 0.5           | 0.5 (csr and ssr)         |

## MSRV

The minimum supported rust version is `1.65`
