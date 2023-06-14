use crate::app::models::cutting_table::CuttingTable;
use crate::app::models::fabric::Fabric;
use crate::app::models::fabric_cut::{FabricCutPiece, PieceStatus, get_optional_pos_x, Vertex, get_optional_pos_y, PositionedRectangle};
use sycamore::prelude::*;

#[component]
pub fn FabricCutPage<G: Html>(cx: Scope<'_>) -> View<G> {
    let current_cutting_table = String::from("2");
    let cutting_tables = vec![
        CuttingTable {
            id: 1,
            name: String::from("Mesa A"),
            width: 1800,
            length: 2800,
        },
        CuttingTable {
            id: 2,
            name: String::from("Mesa B"),
            width: 1500,
            length: 3000,
        },
        CuttingTable {
            id: 3,
            name: String::from("Mesa C"),
            width: 200,
            length: 4000,
        },
    ];
    let current_fabric = String::from("3");
    let fabrics = vec![
        Fabric {
            id: 1,
            name: String::from("Tecido 1"),
            manufacturer: String::from("Fabricante 1"),
            width: 1500,
            code: String::from("xyz001"),
        },
        Fabric {
            id: 2,
            name: String::from("Tecido 2"),
            manufacturer: String::from("Fabricante 1"),
            width: 1800,
            code: String::from("xyz002"),
        },
        Fabric {
            id: 3,
            name: String::from("Tecido 3"),
            manufacturer: String::from("Fabricante 2"),
            width: 1500,
            code: String::from("wlw001"),
        },
        Fabric {
            id: 4,
            name: String::from("Tecido 4"),
            manufacturer: String::from("Fabricante 2"),
            width: 1800,
            code: String::from("wlw002"),
        },
    ];
    let pieces = vec![
        FabricCutPiece {
            id: 1,
            width: 160,
            length: 60,
            status:
                PieceStatus::ProhibitedArea { 
                    position: Vertex {
                        pos_x: 0, 
                        pos_y: 0 
                    }
                }
        },
        FabricCutPiece {
            id: 2,
            width: 120,
            length: 40,
            status: PieceStatus::Fit {
                position: Vertex {
                    pos_x: 0, 
                    pos_y: 60
                }
            },
        },
        FabricCutPiece {
            id: 3,
            width: 40,
            length: 70,
            status: PieceStatus::Fit {
                position: Vertex {
                    pos_x: 160, 
                    pos_y: 0
                }
            },
        },
        FabricCutPiece {
            id: 4,
            width: 20,
            length: 40,
            status: PieceStatus::Fit {
                position: Vertex {
                    pos_x: 130,
                    pos_y: 60
                }
            },
        },
        FabricCutPiece {
            id: 5,
            width: 20,
            length: 40,
            status: PieceStatus::DidNotFit
        },
    ];

    let active_panel = create_signal(cx, 3);
    let cutting_table_list = create_signal(cx, cutting_tables);
    let selected_cutting_table = create_signal(cx, current_cutting_table);
    let fabric_list = create_signal(cx, fabrics);
    let selected_fabric = create_signal(cx, current_fabric);
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

    let fit_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    let prohibited_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    let showcase_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    create_effect(cx, move || {
        let showcase_list: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
        let PiecesToDraw {
            fit_list, 
            prohibited_list, 
            showcase_list 
        } = separate_pieces(&piece_list.get());
        fit_list_to_draw.set(fit_list);
        prohibited_list_to_draw.set(prohibited_list);
        showcase_list_to_draw.set(showcase_list);
    });

    let set_config_panel_active = |_| active_panel.set(1);
    let set_info_panel_active = |_| active_panel.set(2);
    let set_pieces_panel_active = |_| active_panel.set(3);

    let export_g_code = |_| {
        todo!();
    };

    let import_layout = |_| {
        todo!();
    };

    let export_layout = |_| {
        todo!();
    };

    view! { cx,
            div(class="columns mx-1") {
                div(class="panel ml-2 mt-3", style="width:300px") {
                    header(class="panel-heading has-background-grey-lighter") { "Disposição" }
                    div(class="panel-block is-flex") {
                        svg (
                            xmlns="http://www.w3.org/2000/svg", 
                            aria-label="Flickr", role="img", 
                            viewBox=format!("0 0 {} {}", (defined_width.get().to_string()), (max_length.get().to_string())), 
                            style="border:1px solid #000000;" ) {
                            rect(
                                width=(defined_width.get().to_string()), 
                                height=(defined_length.get().to_string()), 
                                x="0", 
                                y="0", 
                                style="fill:rgb(100,100,100);stroke-width:1;stroke:rgb(0,0,0)") {}
                            Keyed(
                                iterable=fit_list_to_draw,
                                view=|cx, item| view! { cx,
                                    svg () {
                                        rect(
                                            width=(item.width.to_string()), 
                                            height=(item.length.to_string()), 
                                            x=(item.pos_x.to_string()), 
                                            y=(item.pos_y.to_string()), 
                                            style="fill:rgb(0,255,0);stroke-width:1;stroke:rgb(0,0,0)") {}
                                    }
                                },
                                key=|item| (item.id.to_string()),
                            )
                            Keyed(
                                iterable=prohibited_list_to_draw,
                                view=|cx, item| view! { cx,
                                    svg () {
                                        rect(
                                            width=(item.width.to_string()), 
                                            height=(item.length.to_string()), 
                                            x=(item.pos_x.to_string()), 
                                            y=(item.pos_y.to_string()), 
                                            style="fill:rgb(0,0,0);stroke-width:1;stroke:rgb(0,0,0)") {}
                                    }
                                },
                                key=|item| (item.id.to_string()),
                            )
                            Keyed(
                                iterable=showcase_list_to_draw,
                                view=|cx, item| view! { cx,
                                    svg () {
                                        rect(
                                            width=(item.width.to_string()), 
                                            height=(item.length.to_string()), 
                                            x=(item.pos_x.to_string()), 
                                            y=(item.pos_y.to_string()), 
                                            style="fill:rgb(255,255,0);stroke-width:1;stroke:rgb(0,0,0)"
                                        ) {}
                                    }
                                },
                                key=|item| (item.id.to_string()),
                            )
                        }
                    }
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
                                            select(bind:value=selected_cutting_table) {
                                                Keyed(
                                                    iterable=cutting_table_list,
                                                    view=move |cx, item| view! { cx,
                                                        FabricCutCuttingTableItem(table=item) {}
                                                    },
                                                    key=|item| item.id,
                                                )

                                            }
                                        }
                                    }
                                }
                                div(class="field") {
                                    label(class="label") { "Tecido"}
                                    div (class="control")  {
                                        div (class="select is-fullwidth") {
                                            select(bind:value=selected_fabric) {
                                                Keyed(
                                                    iterable=fabric_list,
                                                    view=move |cx, item| view! { cx,
                                                        FabricCutFabricItem(fabric=item) {}
                                                    },
                                                    key=|item| item.id,
                                                )

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
                        div(id="collapsible-card-info", class=(match *active_panel.get() {2 => "",_ => "is-collapsible"})) {
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
                                button(class="card-footer-item button is-success m-2", on:click=export_g_code) {
                                    "Gerar código G"
                                }
                                button(class="card-footer-item button is-link m-2", on:click=import_layout) {
                                    "Importar disposição"
                                }
                                button(class="card-footer-item button is-warning m-2", on:click=export_layout) {
                                    "Exportar disposição"
                                }
                            }
                        }
                    }
                    div(class="card") {
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
                        div(id="collapsible-card-pieces", class=(match *active_panel.get() {3 => "", _ => "is-collapsible"})) {
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
                                span(class="card-footer-item mx-1 has-text-white has-background-success") { "Couberam" }
                                span(class="card-footer-item mx-1 has-text-white has-background-danger") { "Sobraram" }
                                span(class="card-footer-item mx-1 has-background-warning") { "Mostruário" }
                                span(class="card-footer-item mx-1 has-text-white has-background-black") { "Área proibida" }
                            }
                        }
                    }

                }
        }
    }
}

#[derive(Props)]
pub struct FabricCutCuttingTableItemProps {
    table: CuttingTable,
}

#[component]
pub fn FabricCutCuttingTableItem<G: Html>(
    cx: Scope,
    props: FabricCutCuttingTableItemProps,
) -> View<G> {
    let item = create_ref(cx, props.table);
    let id = item.id;
    let text = format!(
        "{} ({}mm x {}mm)",
        item.name,
        item.width.clone(),
        item.length.clone()
    );
    view! { cx,
        option(value=id) { (text) }
    }
}

#[derive(Props)]
pub struct FabricCutFabricItemProps {
    fabric: Fabric,
}

#[component]
pub fn FabricCutFabricItem<G: Html>(cx: Scope, props: FabricCutFabricItemProps) -> View<G> {
    let item = create_ref(cx, props.fabric);
    let id = item.id;
    let text = format!("{} ({}mm)", item.name, item.width.clone());
    view! { cx,
        option(value=id) { (text) }
    }
}

#[component(inline_props)]
pub fn FabricCutPieceItem<G: Html>(cx: Scope, piece: FabricCutPiece) -> View<G> {
    let item = create_ref(cx, piece);
    let status_class = get_piece_status_style(&item.status);
    view! { cx,
        tr(class=(status_class)) {
            td (style="vertical-align:middle;") { (item.id.clone()) }
            td (style="vertical-align:middle;") { (item.width.clone())  }
            td (style="vertical-align:middle;") { (item.length.clone()) }
            td (style="vertical-align:middle;") { (get_optional_pos_x(item)) }
            td (style="vertical-align:middle;") { (get_optional_pos_y(item)) }
            td (style="vertical-align:middle;") {
                a(class="button is-responsive is-info", href=(format!("/fabric-cut/{}", item.id ))) { "Editar" }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct PiecesToDraw {
    pub fit_list: Vec<PositionedRectangle>,
    pub prohibited_list: Vec<PositionedRectangle>,
    pub showcase_list: Vec<PositionedRectangle>,
}

fn separate_pieces(piece_list: &Vec<FabricCutPiece>) -> PiecesToDraw {
    let mut fit_list: Vec<PositionedRectangle> = Vec::new();
    let mut prohibited_list: Vec<PositionedRectangle> = Vec::new();
    let mut showcase_list: Vec<PositionedRectangle> = Vec::new();
    
    piece_list.iter().for_each(
        |item| {
            let status = item.status.clone();
            match status {
                PieceStatus::Fit { position } => fit_list.push(
                    PositionedRectangle { 
                        id: item.id, 
                        width: item.width, 
                        length: item.length, 
                        pos_x: position.pos_x,
                        pos_y: position.pos_y 
                    }
                ),
                PieceStatus::DidNotFit => (),
                PieceStatus::ProhibitedArea { position } => prohibited_list.push(
                    PositionedRectangle { 
                        id: item.id, 
                        width: item.width, 
                        length: item.length, 
                        pos_x: position.pos_x,
                        pos_y: position.pos_y 
                    }
                ),
                PieceStatus::Showcase { position_list } => position_list.iter().for_each(
                    |position| {
                        showcase_list.push(
                            PositionedRectangle { 
                                id: item.id, 
                                width: item.width, 
                                length: item.length, 
                                pos_x: position.pos_x,
                                pos_y: position.pos_y 
                            }
                        )
                    }
                )
            }
        }  
    );

    PiecesToDraw {
        fit_list,
        prohibited_list,
        showcase_list,
    }
}

#[component(inline_props)]
pub fn DrawPieceItem<G: Html>(cx: Scope, piece: FabricCutPiece) -> View<G> {
    let item = create_ref(cx, piece);
    let status_class = get_piece_status_style(&item.status);

    view! { cx,
        rect(width="40", height="30", x="0", y="0", style="fill:rgb(0,0,255);stroke-width:1;stroke:rgb(0,0,0)") {}
        }
    }

fn get_piece_status_style(status: &PieceStatus) -> &str{
    match status {
        PieceStatus::ProhibitedArea { position: _ } => "has-text-white has-background-black",
        PieceStatus::DidNotFit => "has-text-white has-background-danger",
        PieceStatus::Showcase { position_list: _ }=> "has-background-warning",
        PieceStatus::Fit { position: _ } => "has-text-white has-background-success"
    }
}