use sycamore::prelude::*;

#[component]
pub fn HomePage<G: Html>(cx: Scope) -> View<G> {    view! { cx,
        div(class="container") {
            h1 (class="title is-2") { "Sobre a empresa"}
            section {
                p { "Teste sobre texto"}
            }
        }
    }
}