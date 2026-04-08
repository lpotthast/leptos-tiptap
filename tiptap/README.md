# Bundling the Tiptap browser module

Install the pinned dependencies from the committed lockfile

    npm ci

This project uses two separate flows:

- `just build` for reproducible rebuilds from `package-lock.json`
- `just update-tiptap` when intentionally upgrading the npm dependencies and regenerating the checked-in bundle
- `npm run typecheck` for adapter-level TypeScript validation
- `npm test` for adapter-level unit tests

Bundle the browser module with esbuild

    npm run build

The generated JS file

    ../leptos-tiptap-build/dist/tiptap.js

contains the bundled Tiptap runtime, the bundled extensions, and this project's JS adapter layer.

The source entrypoints live in `adapter.ts` and `adapter.test.ts`.

Serve it at `/js/tiptap.js`. If you want to preload it in HTML, use module preload:

    <link rel="modulepreload" href="/js/tiptap.js">
