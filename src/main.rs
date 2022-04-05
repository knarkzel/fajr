use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.spawn(async move {
        let rs = reqwest::Client::new()
            .get("https://api.pray.zone/v2/times/today.json?city=oslo")
            .send()
            .await;
        log::debug!("{rs:?}");
    });
    
    cx.render(rsx! {
        button { class: "button is-primary",
            "Click me"
        }
    })
}
