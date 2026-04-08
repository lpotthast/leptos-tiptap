# leptos-tiptap-build

Build dependencies for the [leptos-tiptap](https://crates.io/crates/leptos-tiptap) crate.

Check out the leptos-tiptap [repository](https://github.com/lpotthast/leptos-tiptap) for further instructions on how to
use this dependency.

This crate embeds the following assets:

- `TIPTAP_JS`: the bundled and minified ESM browser module served as `/js/tiptap.js`. It contains the Tiptap runtime,
  the bundled extensions, and this project's JS adapter layer used by the Rust bridge.

## Changelog

0.3.0 - UNRELEASED

- Changed the packaging strategy. Now, `tiptap.js` is not only our adapter code, but instead the Tiptap runtime and
  adapter combined. You only need to include this file in your build.
- Updated Tiptap and plugins from `2.12.0` to `2.27.2`.

0.2.8

- Updated Tiptap and plugins from `2.10.4` to `2.12.0`.

0.2.7

- Updated Tiptap and plugins from `2.2.0` to `2.10.4`.
- Added support for links, embedded YouTube videos as well as ordered and unordered lists.

0.2.6

- Removed unused dependency.

0.2.5

- Updated Tiptap and plugins from `2.1.12` to `2.2.0`.

0.2.4

- Updated Tiptap and plugins from `2.1.8` to `2.1.12`.

0.2.3

- Updated Tiptap and plugins from `2.0.3` to `2.1.8`.

0.2.2

- Updated Tiptap and plugins from `2.0.3` to `2.1.7`.

## MSRV

The minimum supported rust version is `1.60`.
