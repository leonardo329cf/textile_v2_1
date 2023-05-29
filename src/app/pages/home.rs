use sycamore::{prelude::*, futures::spawn_local_scoped};

use crate::app::{services::about_service::get_about};

#[component]
pub async fn HomePage<G: Html>(cx: Scope<'_>) -> View<G> {
    let error_msg = create_signal(cx, Option::<String>::None);

    let about_msg = create_signal(cx, String::new());

    let about = move || {
        spawn_local_scoped(cx, async move {
            let new_msg =
                get_about().await;

            match new_msg {
                Ok(value) => about_msg.set(value),
                Err(error) => error_msg.set(Option::Some(error.message))
            };
        })
    };
    about();
    view! { cx,
        div(class="container") {
            (
                match (*error_msg.get()).clone() {
                    Some(msg) => 
                        view!(cx ,
                            div(class="notification is-danger") {
                                button(class="delete", on:click= |_| error_msg.set(Option::None))
                                p { (msg) }
                            }
                        ),
                    None => 
                        view!(cx ,
                        ),
                }
            )
            div(class="container") {
                h1 (class="title is-2") { "Sobre a empresa"}
                
                section {
                    p { (about_msg.get()) }
                }
            }
        }
    }
}