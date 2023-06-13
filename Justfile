# Lists all available commands.
list:
  just --list

# Perform a full build of the tiptap bundle.
build:
  just install
  just update
  just bundle
  just minify

# Run `npm install`
install:
  cd tiptap && npm install

# Run `npm update`
update:
  cd tiptap && npm update

# Bundle tiptap into a single JS file -> leptos-tiptap-build/dist/tiptap-bundle.js
bundle:
  browserify tiptap/main.js -o leptos-tiptap-build/dist/tiptap-bundle.js

# Minify a previously created tiptap bundle -> leptos-tiptap-build/dist/tiptap-bundle.min.js
minify:
  uglifyjs --compress --mangle --output leptos-tiptap-build/dist/tiptap-bundle.min.js -- leptos-tiptap-build/dist/tiptap-bundle.js
