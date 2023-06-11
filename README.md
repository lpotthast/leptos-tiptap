# Bundling tiptap into a single static js file

Beginning with an empty directory...

Create an initial package.json with

    npm init

Install browserify

    npm install -g browserify
    npm install -g uglify-js

Install your packages

    npm install @tiptap/core
    npm install @tiptap/starter-kit
    npm install @tiptap/extension-highlight
    npm install @tiptap/extension-image
    npm install @tiptap/extension-text-align

Create a file called `main.js` which `require`s your previously installed dependencies.

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

Bundle them together with

    browserify main.js -o dist/tiptap-bundle.js

Minify the output with uglify-js

    uglifyjs --compress --mangle --output dist/tiptap-bundle.min.js -- dist/tiptap-bundle.js

You can now include

    dist/tiptap-bundle.min.js

in your web page! :)
