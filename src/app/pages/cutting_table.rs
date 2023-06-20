use sycamore::{component, futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

use crate::app::{
    log,
    models::cutting_table::{CuttingTable, CuttingTableCreate},
    services::cutting_table_service::{
        create_cutting_table, delete_cutting_table, get_all_cutting_table, get_cutting_table_by_id,
        update_cutting_table,
    },
};

#[component(inline_props)]
fn CuttingTableItem<G: Html>(cx: Scope, cutting_table: CuttingTable) -> View<G> {
    let item = create_ref(cx, cutting_table);
    view! { cx,
        tr(class="p-7") {
            td (style="width:5%; vertical-align:middle;") { (item.id.clone()) }
            td (style="width:30%; vertical-align:middle;") { (item.name.clone())  }
            td (style="width:10%; vertical-align:middle;") { (item.width.clone()) }
            td (style="width:10%; vertical-align:middle;") { (item.length.clone()) }
            td (style="width:10%; vertical-align:middle;") {
                a(class="button is-medium is-fullwidth is-success", href=(format!("/cutting-table/{}", item.id ))) { "Editar" }
            }
        }
    }
}

#[component]
pub fn CuttingTableListPage<G: Html>(cx: Scope<'_>) -> View<G> {
    let cutting_table_list: &Signal<Vec<CuttingTable>> =
        create_signal(cx, Vec::<CuttingTable>::new());

    let fetch_all_cutting_table = move || {
        spawn_local_scoped(cx, async move {
            let new_cutting_table_list = get_all_cutting_table().await;

            match new_cutting_table_list {
                Ok(value) => cutting_table_list.set(value),
                Err(_error) => cutting_table_list.set(Vec::<CuttingTable>::new()),
            };
        })
    };

    fetch_all_cutting_table();

    let fetch_all_cutting_table_click = move |_| {
        spawn_local_scoped(cx, async move {
            cutting_table_list.set(Vec::<CuttingTable>::new());
            let new_cutting_table_list = get_all_cutting_table().await;

            match new_cutting_table_list {
                Ok(value) => cutting_table_list.set(value),
                Err(_error) => cutting_table_list.set(Vec::<CuttingTable>::new()),
            };
        })
    };

    view! { cx,
        div(class="container") {
            div (class="level-left mb-2") {
                h1 (class="title mb-0 is-2 level-item") { "Mesas de Corte" }
                div (class="level-item") {
                    button (class="button is-medium", on:click=fetch_all_cutting_table_click) { "Recarregar" }
                }
            }
            div(class="table-container") {
                table(class="table is-striped is-fullwidth") {
                    thead {
                        tr {
                            th(style="width:5%; vertical-align:middle;") { "ID" }
                            th(style="width:50%; vertical-align:middle;") { "Nome" }
                            th(style="width:15%; vertical-align:middle;") { "Largura (mm)" }
                            th(style="width:20%; vertical-align:middle;") { "Comprimento (mm)" }
                            th(style="width:10%; vertical-align:middle;") {
                                a(class="button is-medium is-success is-fullwidth", href="/cutting-table/0") { "Novo" }
                            }
                        }
                    }
                    tbody {
                        Keyed(
                            iterable=cutting_table_list,
                            view=move |cx, item| view! { cx,
                                CuttingTableItem( cutting_table = item )
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
pub struct CuttingTableItemProps {
    id: i32,
}

#[component]
pub fn CuttingTableItemPage<G: Html>(cx: Scope<'_>, props: CuttingTableItemProps) -> View<G> {
    let id = create_signal(cx, 0.0);
    let name = create_signal(cx, String::new());
    let width = create_signal(cx, 0.0);
    let length = create_signal(cx, 0.0);
    let error_message = create_signal(cx, String::new());

    let param_id = props.id;
    if param_id > 0 {
        spawn_local_scoped(cx, async move {
            if let Ok(item) = get_cutting_table_by_id(param_id).await {
                id.set(item.id as f64);
                name.set(item.name);
                width.set(item.width as f64);
                length.set(item.length as f64);
            }
        })
    }

    let save_item = move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = id.get().as_ref().round() as i32;
            let param_name = name.get().as_ref().clone();
            let param_width = width.get().as_ref().round() as i32;
            let param_length = length.get().as_ref().round() as i32;
            let response = match param_id {
                0 => {
                    let item = CuttingTableCreate {
                        name: param_name,
                        width: param_width,
                        length: param_length,
                    };
                    create_cutting_table(item).await
                }
                _ => {
                    let item = CuttingTable {
                        id: param_id,
                        name: param_name,
                        width: param_width,
                        length: param_length,
                    };
                    update_cutting_table(item).await
                }
            };
            match response {
                Ok(_) => navigate("/cutting-table"),
                Err(e) => error_message.set(e.message),
            }
        });
    };

    let delete_item = move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = id.get().as_ref().round() as i32;
            let response = delete_cutting_table(param_id).await;
            match response {
                Ok(_) => navigate("/cutting-table"),
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
                            p(class="modal-card-title level-left") { "Editar de Mesa" }
                        )
                    } else {
                        view! { cx, p(class="modal-card-title level-left") { "Criar de Mesa" }} // Now you don't
                    })
                    div(class="level-item level-right") {
                        a(class="button is-medium is-warning", href="/cutting-table") { "Cancelar" }
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
                        label(class="label") { "Largura (mm)" }
                        div(class="control") {
                            input(class="input", type="number", placeholder="identificador", bind:valueAsNumber=width)
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Comprimento (mm)" }
                        div(class="control") {
                            input(class="input", type="number", placeholder="identificador", bind:valueAsNumber=length)
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
