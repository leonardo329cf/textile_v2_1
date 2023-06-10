use sycamore::{prelude::*, component, futures::spawn_local_scoped};

use crate::app::{models::fabric::Fabric, services::fabric_service::get_all_fabric};


#[component(inline_props)]
fn FabricItem<G: Html>(cx: Scope, fabric: Fabric) -> View<G> {
    let item = create_ref(cx, fabric);
    view! { cx,
        tr {
            td { (item.id.clone()) }
            td { (item.name.clone())  }
            td { (item.manufacturer.clone()) }
            td { (item.width.clone()) }
            td { (item.code.clone()) }
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

    view! { cx,
        div(class="container") {
            h1 (class="title is-2") { "Tecido" }
            div(class="table-container") {
                table(class="table is-striped is-fullwidth") {
                    thead {
                        tr {
                            th(style="width:5%") { "ID" }
                            th(style="width:40%") { "Nome" }
                            th(style="width:30%") { "Fabricante" }
                            th(style="width:10%") { "Largura (mm)" }
                            th(style="width:10%") { "Código" }
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
