# Lists all available commands.
list:
  just --list

# Perform a full build of the tiptap bundle.
build:
  just install-prerequisites
  just install-tiptap
  just update-tiptap
  just bundle-tiptap
  just minify-tiptap

# Install dependencies
install-prerequisites:
  npm install -g browserify
  npm install -g uglify-js

# Run `npm install`
install-tiptap:
  cd tiptap && npm install

# Run `npm update`
update-tiptap:
  cd tiptap && npm update

# Bundle tiptap into a single JS file -> leptos-tiptap-build/dist/tiptap-bundle.js
bundle-tiptap:
  browserify tiptap/main.js -o leptos-tiptap-build/dist/tiptap-bundle.js

# Minify a previously created tiptap bundle -> leptos-tiptap-build/dist/tiptap-bundle.min.js
minify-tiptap:
  uglifyjs --compress --mangle --output leptos-tiptap-build/dist/tiptap-bundle.min.js -- leptos-tiptap-build/dist/tiptap-bundle.js

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv --min "2021" --path leptos-tiptap
    cargo msrv --min "2021" --path leptos-tiptap-build
