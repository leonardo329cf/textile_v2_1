use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::app::services::about_service::get_about;

#[component]
pub fn HomePage<G: Html>(cx: Scope<'_>) -> View<G> {
    let error_msg = create_signal(cx, Option::<String>::None);

    let about_msg = create_signal(cx, String::new());

    let about = move || {
        spawn_local_scoped(cx, async move {
            let new_msg = get_about().await;

            match new_msg {
                Ok(value) => about_msg.set(value),
                Err(error) => error_msg.set(Option::Some(error.message)),
            };
        })
    };
    about();
    view! { cx,
        div(class="container-fluid") {
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
            div(class="has-text-centered") {
                figure(class="image is-inline-block") {
                    img(src="images/logo.png") {}
                }
            }
            div(class="container mt-4") {
                section {
                    p { (about_msg.get()) }
                }
            }
        }
    }
}
