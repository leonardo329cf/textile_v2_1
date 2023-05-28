use sycamore::prelude::*;

#[component]
pub fn NotFoundPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="container") {
            h1 { "Not Found" }
            div(class="col-lg-8 px-0") {
                p { "Página não encontrada." }
            }
        }
    }
}