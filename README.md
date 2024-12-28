# leptos-tiptap

Enables the integration of [Tiptap](https://tiptap.dev/) instances into your [leptos](https://leptos.dev/) projects.

Currently used tiptap version: `2.10.4`, when using leptos-tiptap-build `0.2.6`

Currently used tiptap extensions:

- "@tiptap/core": "^2.10.4",
- "@tiptap/extension-highlight": "^2.10.4",
- "@tiptap/extension-image": "^2.10.4",
- "@tiptap/extension-youtube": "^2.10.4",
- "@tiptap/extension-link": "^2.10.4",
- "@tiptap/extension-text-align": "^2.10.4,
- "@tiptap/starter-kit": "^2.10.4"

This repository contains:

| Dir                 | What is it for?                                                                                                                                                                                                                              |
|---------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| leptos-tiptap       | The main dependency other leptos projects can depend on. It provides the `<TiptapInstance>` component through which a tiptap instance is managed automatically.                                                                              |
| leptos-tiptap-build | This dependency can be used in `build.rs` scrips. It provides the correctly compiled tiptap JS bundle which must be included in your application to work properly. Check out the example in `leptos-tiptap` to see how this can be achieved. |
| tiptap              | Build process for the tiptap JS bundle. Can be triggered by calling `just build`. Build output is automatically saved to `leptos-tiptap-build` and picked up by the example in `leptos-tiptap`.                                              |
