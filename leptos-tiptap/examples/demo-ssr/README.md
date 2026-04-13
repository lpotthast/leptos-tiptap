# demo-ssr

Install cargo-leptos with

    cargo install cargo-leptos

And run the demo with

    cargo leptos watch

Browser integration tests are run from the library crate with

    cd ../.. && cargo test --test browser_test -- --nocapture
