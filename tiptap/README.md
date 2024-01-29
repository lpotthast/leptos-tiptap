# Bundling the tiptap NPM packages into a single static js file

Beginning with an empty directory...

Create an initial package.json with

    npm init

Install browserify and uglify-js globally

    npm install -g browserify
    npm install -g uglify-js

Install the required packages

    npm install @tiptap/core
    npm install @tiptap/starter-kit
    npm install @tiptap/extension-highlight
    npm install @tiptap/extension-image
    npm install @tiptap/extension-text-align

Create a file called `main.js` which `require`s your previously installed dependencies.

You may make these dependencies available, by adding them as new `global.window` properties or through other means.

    var TipTap = require('@tiptap/core')
    var TipTapStarterKit = require('@tiptap/starter-kit')
    var TipTapHighlight = require('@tiptap/extension-highlight')
    var TipTapTextAlign = require('@tiptap/extension-text-align')
    var TipTapImage = require('@tiptap/extension-image')
    global.window.TipTap = TipTap
    global.window.TipTapStarterKit = TipTapStarterKit
    global.window.TipTapHighlight = TipTapHighlight
    global.window.TipTapTextAlign = TipTapTextAlign
    global.window.TipTapImage = TipTapImage

Create a single JS bundle using the browserify tool

    browserify main.js -o ../leptos-tiptap-build/dist/tiptap-bundle.js

And minify the output with uglify-js

    uglifyjs --compress --mangle --output ../leptos-tiptap-build/dist/tiptap-bundle.min.js -- ../leptos-tiptap-build/dist/tiptap-bundle.js

The generated JS files

    ../leptos-tiptap-build/dist/tiptap-bundle.js
    ../leptos-tiptap-build/dist/tiptap-bundle.min.js

can now be used with a simple HTML script element

    <script type="module" src="/js/tiptap-bundle.min.js"></script>
