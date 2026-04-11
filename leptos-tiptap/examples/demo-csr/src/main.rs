use demo_app::DemoApp;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <DemoApp/> }
    })
}
