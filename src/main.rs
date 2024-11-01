#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| { rsx! { "Loading..." } },
            Text {}
        }
    }
}

#[component]
fn Text() -> Element {
    let Some(text) = use_server_future(get_server_data)?.value()() else {
        return rsx! { "Loading..." };
    };
    let Ok(text) = text else {
        return rsx! { "Error!" };
    };
    rsx! { "{text}" }
}

// #[component]
// fn SuspendText() -> Element {
//     let text = use_server_future(get_server_data)?.suspend()?().unwrap();
//     rsx! { "{text}" }
// }

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    Ok("Hello from the server!".to_string())
}
