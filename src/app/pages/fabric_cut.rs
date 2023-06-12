use crate::app::models::fabric_cut::FabricCutPiece;
use sycamore::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub active_panel: RcSignal<i32>,
}

#[component]
pub fn FabricCutPage<G: Html>(cx: Scope<'_>) -> View<G> {
    let pieces = vec![
        FabricCutPiece {
            id: 1,
            length: 500,
            width: 500,
            pos_x: 1,
            pos_y: 1,
            status: 1,
        },
        FabricCutPiece {
            id: 2,
            length: 1200,
            width: 1800,
            pos_x: 2,
            pos_y: 2,
            status: 1,
        },
        FabricCutPiece {
            id: 3,
            length: 500,
            width: 500,
            pos_x: 3,
            pos_y: 3,
            status: 3,
        },
        FabricCutPiece {
            id: 4,
            length: 500,
            width: 500,
            pos_x: 1500,
            pos_y: 1500,
            status: 4,
        },
        FabricCutPiece {
            id: 5,
            length: 1200,
            width: 1800,
            pos_x: 5,
            pos_y: 5,
            status: 1,
        },
    ];

    let active_panel = create_signal(cx, 2);
    let selected_cutting_table = create_signal(cx, 2.0);
    let selected_fabric = create_signal(cx, 0.0);
    let defined_width = create_signal(cx, 0.0);
    let max_length = create_signal(cx, 0.0);
    let spacing = create_signal(cx, 0.0);
    let defined_length = create_signal(cx, 0.0);
    let piece_list = create_signal(cx, pieces);
    let is_right_align = create_signal(cx, false);
    let total_area = create_signal(cx, 9.0);
    let used_area = create_signal(cx, 4.57);
    let percentage_use = create_signal(cx, 50.78);
    let length = create_signal(cx, 1800.0);

    let set_config_panel_active = |_| active_panel.set(1);
    let set_pieces_panel_active = |_| active_panel.set(2);
    let set_info_panel_active = |_| active_panel.set(3);

    view! { cx,
        div(class="container") {
            div(class="columns") {
                div(class="column is-4") {
                    "Teste"
                }
                div(class="column") {
                    div(class="card") {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Configuração"
                            }
                            button(href="#collapsible-card-config", class="card-header-icon is-hidden-fullscreen", aria-label="more options", on:click=set_config_panel_active) {
                                span(class="icon") {
                                    i(class="fas fa-angle-down", aria-hidden="true") {}
                                }
                            }
                        }
                        div(id="collapsible-card-config", class=(match *active_panel.get() {1 => "",_ => "is-collapsible"})) {
                            div(class="card-content") {
                                div(class="field") {
                                    label(class="label") { "Mesa de corte"}
                                    div (class="control")  {
                                        div (class="select is-fullwidth") {
                                            select(bind:valueAsNumber=selected_cutting_table) {
                                                option { "Mesa média (1800mm x 5000mm)" }
                                                option { "Mesa grande (2800mm x 7000mm)" }
                                            }
                                        }
                                    }
                                }
                                div(class="field") {
                                    label(class="label") { "Tecido"}
                                    div (class="control")  {
                                        div (class="select is-fullwidth") {
                                            select(bind:valueAsNumber=selected_fabric) {
                                                option { "Tecido azul (1800mm x 25000mm)" }
                                                option { "Tecido negro (2800mm x 35000mm)" }
                                            }
                                        }
                                    }
                                }
                                div(class="columns") {
                                    div(class="column field") {
                                        label(class="label") { "Largura definida (mm)"}
                                        div (class="control")  {
                                            input(class="input", type="number", placeholder="Number input", bind:valueAsNumber=defined_width) {}
                                        }
                                    }
                                    div(class="column field") {
                                        label(class="label") { "Comprimento máximo (mm)"}
                                        div (class="control")  {
                                            input(class="input", type="number", placeholder="Number input", bind:valueAsNumber=max_length) {}
                                        }
                                    }
                                }
                                div(class="columns") {
                                    div(class="column is-3 field") {
                                        label(class="label") { "Espaçamento(mm)" }
                                        div (class="control")  {
                                            input(class="input", type="number", placeholder="Number input", bind:valueAsNumber=spacing) {}
                                        }
                                    }
                                    div(class="column is-5 field") {
                                        label(class="label") { "Comprimento definido (mm)"}
                                        div (class="control")  {
                                            input(class="input", type="number", placeholder="Number input", bind:valueAsNumber=defined_length) {}
                                        }
                                    }
                                    div(class="column is-4 field") {
                                        label(class="label") { "Opções"}
                                        div(class="control") {
                                            label(class="checkbox mt-2") {
                                                input(type="checkbox", bind:checked=is_right_align) {}
                                                " Organizar a direita"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div(class="card my-2") {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Peças"
                            }
                            button(href="#collapsible-card-pieces", class="card-header-icon is-hidden-fullscreen is-active", aria-label="more options", on:click=set_pieces_panel_active) {
                                span(class="icon") {
                                    i(class="fas fa-angle-down", aria-hidden="true") {}
                                }
                            }
                        }
                        div(id="collapsible-card-pieces", class=(match *active_panel.get() {2 => "", _ => "is-collapsible"})) {
                            div(class="card-content") {
                                table(class="table is-striped is-fullwidth") {
                                    thead {
                                        tr {
                                            th(style="5%") { "Id" }
                                            th(style="20%") { "Largura" }
                                            th(style="20%") { "Comprimento" }
                                            th(style="20%") { "Posição X" }
                                            th(style="20%") { "Posição Y" }
                                            th(style="10%") {
                                                a(class="button is-responsive is-success") { "Nova" }
                                            }
                                        }
                                    }
                                    tbody {
                                        Keyed(
                                            iterable=piece_list,
                                            view=move |cx, item| view! { cx,
                                                FabricCutPieceItem(piece=item) {}
                                            },
                                            key=|item| item.id,
                                        )
                                    }
                                }
                            }
                            footer(class="card-footer m-4") {
                                span(class="card-footer-item mx-1 has-text-white has-background-link") { "Couberam" }
                                span(class="card-footer-item mx-1 has-text-white has-background-danger") { "Sobraram" }
                                span(class="card-footer-item mx-1 has-background-warning") { "Mostruário" }
                                span(class="card-footer-item mx-1 has-text-white has-background-success") { "Área proibida" }
                            }
                        }
                    }
                    div(class="card") {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Informações"
                            }
                            button(href="#collapsible-card-info", class="card-header-icon is-hidden-fullscreen", aria-label="more options", on:click=set_info_panel_active) {
                                span(class="icon") {
                                    i(class="fas fa-angle-down", aria-hidden="true") {}
                                }
                            }
                        }
                        div(id="collapsible-card-info", class=(match *active_panel.get() {3 => "",_ => "is-collapsible"})) {
                            div(class="card-content columns  has-text-centered") {
                                div(class="column is-2") {
                                    label(class="label") { "Área total" }
                                    p { (format!("{} m³", total_area)) }
                                }
                                div(class="column is-3") {
                                    label(class="label") { "Área aproveitada" }
                                    p { (format!("{} m³", used_area)) }
                                }
                                div(class="column is-3") {
                                    label(class="label") { "Aproveitamento" }
                                    p { (format!("{} %", percentage_use)) }
                                }
                                div(class="column is-4") {
                                    label(class="label") { "Comprimento utiliz." }
                                    p { (format!("{} mm", length)) }
                                }
                            }
                            footer(class="card-footer") {
                                button(class="card-footer-item button is-success m-2") {
                                    "Gerar código G"
                                }
                                button(class="card-footer-item button is-link m-2") {
                                    "Importar disposição"
                                }
                                button(class="card-footer-item button is-warning m-2") {
                                    "Exportar disposição"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
pub fn FabricCutPieceItem<G: Html>(cx: Scope, piece: FabricCutPiece) -> View<G> {
    let item = create_ref(cx, piece);
    let status_class = match item.status {
        1 => "has-text-white has-background-link",
        2 => "has-text-white has-background-danger",
        3 => "has-background-warning",
        4 => "has-text-white has-background-success",
        _ => "",
    };
    view! { cx,
        tr(class=(status_class)) {
            td (style="vertical-align:middle;") { (item.id.clone()) }
            td (style="vertical-align:middle;") { (item.width.clone())  }
            td (style="vertical-align:middle;") { (item.length.clone()) }
            td (style="vertical-align:middle;") { (item.pos_x.clone()) }
            td (style="vertical-align:middle;") { (item.pos_y.clone()) }
            td (style="vertical-align:middle;") {
                a(class="button is-responsive is-info", href=(format!("/fabric-cut/{}", item.id ))) { "Editar" }
            }
        }
    }
}
