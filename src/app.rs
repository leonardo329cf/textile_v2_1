mod pages;
mod services;
mod models;

use sycamore::prelude::*;
use sycamore_router::{Router, Route, HistoryIntegration};
use wasm_bindgen::prelude::*;
use pages::home::HomePage;
use pages::not_found::NotFoundPage;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header { AppNav {} }
        main { AppRouter {} }
        footer(class="mt-auto has-background-black has-text-centered") {
            figure(class="image is-inline-block") {
                img(src="images/logo.png") {}
            }
        }
    }
}

#[component]
fn AppNav<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav(id="navbar", class="navbar mb-2", role="navigation", aria-label="main navigation") {
            div(id="navMenu", class="navbar-menu") {
                div(class="navbar-start") {
                    a(class="navbar-item", href="/") { "Sobre" }
                }
            }
        }
    }
}

#[component]
fn AppRouter<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx, (
                    match route.get().as_ref() {
                        AppRoutes::Index => view! { cx, HomePage {} },
                        AppRoutes::NotFound => view! { cx, NotFoundPage {} },
                    }
                )}
            }
        )
    }
}

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
}