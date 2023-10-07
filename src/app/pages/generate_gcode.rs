use sycamore::{web::Html, reactive::{Scope, create_signal}, component, view::View, futures::spawn_local_scoped};
use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::app::services::generate_g_code_service::generate_g_code_file;

#[component]
pub fn GenerateGCodePage< G: Html>(cx: Scope<'_>) -> View<G> {

    let error_message = create_signal(cx, String::new());

    let pull_textile = create_signal(cx, true);

    let name = create_signal(cx, String::new());

    let fill_name_with_date = move |_| {
        name.set(chrono::offset::Local::now().format("%Y-%m-%d_%Hh%Mm%Ss").to_string());
    };

    let generate = move |_| {
        spawn_local_scoped(cx, async move {
            let response = generate_g_code_file(name.get().as_ref().clone(), *pull_textile.get().as_ref()).await;
            match response {
                Ok(_) => navigate("/fabric-cut"),
                Err(e) => error_message.set(e.message),
            }
        })
    };

    view! { cx,
        div(class="modal is-active") {
            div(class="modal-background") {}
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title level-left") { "Gerador de Código G" } 
                    div(class="level-item level-right") {
                        a(class="button is-medium is-warning", href="/fabric-cut") { "Cancelar" }
                    }
                }
                section(class="modal-card-body") {
                            div(class="field") {
                                label(class="label") { "Nome" }
                                div(class="columns") {
                                    div(class="column is-three-quarters") {
                                        div(class="control") {
                                            input(class="input", type="text", placeholder="Nome do arquivo", bind:value=name)
                                        }
                                    }
                                    div(class="column is-one-quarter is-bottom") {
                                        button(class="button is-grey", on:click=fill_name_with_date) {"Gerar Nome"}
                                    }
                            }
                        }
                    div(class="field") {
                        label(class="checkbox") { 
                            input(
                                class="toggle",
                                type="checkbox",
                                bind:checked=pull_textile,
                            )
                            " Puxar tecido"
                        }
                    }
                    

                    div {
                        p(class="has-text-danger") { (error_message.get()) }
                    }
                }
                footer(class="modal-card-foot") {
                    div(class="level container") {
                        div(class="level-rigth") {
                            button(class="button is-medium is-success", on:click=generate) { "Gerar" }
                        }
                    }
                }
            }
        }
    }
}