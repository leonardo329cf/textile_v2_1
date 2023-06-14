use sycamore::{prelude::*, component, futures::spawn_local_scoped};
use sycamore_router::navigate;

use crate::app::{models::fabric::{Fabric, FabricCreate}, services::fabric_service::{get_all_fabric, get_fabric_by_id, create_fabric, update_fabric, delete_fabric}};


#[component(inline_props)]
fn FabricItem<G: Html>(cx: Scope, fabric: Fabric) -> View<G> {
    let item = create_ref(cx, fabric);
    view! { cx,
        tr(class="p-7") {
            td (style="width:5%; vertical-align:middle;") { (item.id.clone()) }
            td (style="width:30%; vertical-align:middle;") { (item.name.clone())  }
            td (style="width:30%; vertical-align:middle;") { (item.manufacturer.clone()) }
            td (style="width:10%; vertical-align:middle;") { (item.width.clone()) }
            td (style="width:15%; vertical-align:middle;") { (item.code.clone()) }
            td (style="width:10%; vertical-align:middle;") {
                a(class="button is-medium is-fullwidth is-success", href=(format!("/fabric/{}", item.id ))) { "Editar" }
            }
        }
    }
}

#[component]
pub fn FabricListPage<G: Html>(cx: Scope<'_>) -> View<G> {
    let fabric_list: &Signal<Vec::<Fabric>> = create_signal(cx, Vec::<Fabric>::new());

    let fetch_all_fabric = move || {
        spawn_local_scoped(cx, async move {
            let new_fabric_list =
                get_all_fabric().await;

            match new_fabric_list {
                Ok(value) => fabric_list.set(value),
                Err(_error) => fabric_list.set(Vec::<Fabric>::new())
            };
        })
    };

    fetch_all_fabric();

    let fetch_all_fabric_click = move |_| {
        spawn_local_scoped(cx, async move {
            fabric_list.set(Vec::<Fabric>::new());
            let new_fabric_list =
                get_all_fabric().await;

            match new_fabric_list {
                Ok(value) => fabric_list.set(value),
                Err(_error) => fabric_list.set(Vec::<Fabric>::new())
            };
        })
    };

    view! { cx,
        div(class="container") {
            div (class="level-left mb-2") {
                h1 (class="title mb-0 is-2 level-item") { "Tecido" }
                div (class="level-item") {
                    button (class="button is-medium", on:click=fetch_all_fabric_click) { "Recarregar" }
                }
            }
            div(class="table-container") {
                table(class="table is-striped is-fullwidth") {
                    thead {
                        tr {
                            th(style="width:5%; vertical-align:middle;") { "ID" }
                            th(style="width:30%; vertical-align:middle;") { "Nome" }
                            th(style="width:30%; vertical-align:middle;") { "Fabricante" }
                            th(style="width:10%; vertical-align:middle;") { "Largura (mm)" }
                            th(style="width:15%; vertical-align:middle;") { "Código" }
                            th(style="width:10%; vertical-align:middle;") {
                                a(class="button is-medium is-success is-fullwidth", href="/fabric/0") { "Novo" }
                            }
                        }
                    }
                    tbody {
                        Keyed(
                            iterable=fabric_list,
                            view=move |cx, item| view! { cx,
                                FabricItem( fabric = item )
                            },
                            key=|item| item.id,
                        )
                    }
                }
            }
        }
    }
}


#[derive(Props)]
pub struct FabricItemProps<> {
    id: i32,
}


#[component]
pub fn FabricItemPage< G: Html>(cx: Scope<'_>, props: FabricItemProps) -> View<G> {
    let id = create_signal(cx, 0.0);
    let name = create_signal(cx, String::new());
    let manufacturer = create_signal(cx, String::new());
    let width = create_signal(cx, 0.0);
    let code = create_signal(cx, String::new());
    let error_message = create_signal(cx, String::new());

    let param_id = props.id;
    if param_id > 0 {
        spawn_local_scoped(cx, async move {
            if let Ok(item) = get_fabric_by_id(param_id).await {
                id.set(item.id as f64);
                name.set(item.name);
                manufacturer.set(item.manufacturer);
                width.set(item.width as f64);
                code.set(item.code);
            }
        })
    }

    let save_item =  move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = id.get().as_ref().round() as i32;
            let param_name = name.get().as_ref().clone();
            let param_manufacturer = manufacturer.get().as_ref().clone();
            let param_width = width.get().as_ref().round() as i32;
            let param_code = code.get().as_ref().clone();
            let response = match param_id {
                0 => {
                    let item = FabricCreate {
                        name: param_name,
                        manufacturer: param_manufacturer,
                        width: param_width,
                        code: param_code,
                    };
                    create_fabric(item).await
                },
                _ => {
                    let item = Fabric {
                        id: param_id,
                        name: param_name,
                        manufacturer: param_manufacturer,
                        width: param_width,
                        code: param_code,
                    };
                    update_fabric(item).await
                }
            };
            match response {
                Ok(_) => {
                    navigate("/fabric")
                },
                Err(e) => error_message.set(e.message),
            }
        });
    };

    let delete_item = move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = id.get().as_ref().round() as i32;
            let response = delete_fabric(param_id).await;
            match response {
                Ok(_) => navigate("/fabric"),
                Err(e) => error_message.set(e.message),
            }
        })
    };

    view! { cx,
        div(class="modal is-active") {
            div(class="modal-background") {}
            div(class="modal-card") {
                header(class="modal-card-head") {
                    (if param_id > 0 {
                        view!(cx, 
                            p(class="modal-card-title level-left") { "Editar de Tecido" }
                        )
                    } else {
                        view! { cx, p(class="modal-card-title level-left") { "Criar de Tecido" }} // Now you don't
                    })
                    div(class="level-item level-right") {
                        a(class="button is-medium is-warning", href="/fabric") { "Cancelar" }
                    }
                }
                section(class="modal-card-body") {
                    (if param_id > 0 {
                        view!(cx, 
                            div(class="field") {
                                label(class="label") { "ID" }
                                div(class="control") {
                                    label(class="label") { (id.get()) }
                                }
                            }
                        )
                    } else {
                        view! { cx, } // Now you don't
                    })
                    div(class="field") {
                        label(class="label") { "Nome" }
                        div(class="control") {
                            input(class="input", type="text", placeholder="nome completo", bind:value=name)
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Fabricante" }
                        div(class="control") {
                            input(class="input", type="text", placeholder="fabricante", bind:value=manufacturer)
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Largura (mm)" }
                        div(class="control") {
                            input(class="input", type="number", placeholder="identificador", bind:valueAsNumber=width)
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Código" }
                        div(class="control") {
                            input(class="input", type="text", placeholder="identificador", bind:value=code)
                        }
                    }
                    div {
                        p(class="has-text-danger") { (error_message.get()) }
                    }
                }
                footer(class="modal-card-foot") {
                    div(class="level container") {
                        div(class="level-rigth") {
                            button(class="button is-medium is-success", on:click=save_item) { "Salvar" }
                        }
                        (if param_id > 0 {
                            view!(cx, 
                                div(class="level-left") {
                                    button(class="button is-medium is-danger", on:click=delete_item) { "Apagar" }
                                })
                        } else {
                            view! { cx, } // Now you don't
                        })
                    }
                }
            }
        }
    }
}