# demo-ssr

Install cargo-leptos with

    cargo install cargo-leptos

And run the demo with

    cargo leptos watch

End-to-end tests live in `end2end/`. Install the test dependencies with

    cd end2end && npm install && npx playwright install chromium

And run them with

    cargo leptos test
