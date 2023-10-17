use sycamore::{prelude::*, futures::spawn_local_scoped};

use crate::app::{services::{cut_disposition_service::{get_cut_disposition_input, set_config_cut_disposition_input, get_config_cut_disposition_input, get_cut_disposition_output}, cutting_table_service::get_all_cutting_table, fabric_service::get_all_fabric, export_import_service::import_disposition}, utils::utils::get_optional_from_boolean_and_value, models::{cut_disposition::{ConfigCutDispositionInput, Rectangle, PositionedRectangle}, cutting_table::CuttingTable, fabric::Fabric}};

enum SelectedPanel {
    Config,
    Pieces,
    Info
}

#[component]
pub fn FabricCutPage<G: Html>(cx: Scope<'_>) -> View<G> {

    let active_panel = create_signal(cx, SelectedPanel::Config);

    let set_config_panel_active = |_| active_panel.set(SelectedPanel::Config);

    let set_pieces_panel_active = |_| active_panel.set(SelectedPanel::Pieces);

    let set_info_panel_active = |_| active_panel.set(SelectedPanel::Info);

    let defined_width = create_signal(cx, 0.0);
    let max_length = create_signal(cx, 0.0);

    let spacing = create_signal(cx, 0.0);
    let spacing_selection = create_signal(cx, false);
    let toggle_spacing_selection = || {
        spacing_selection.set(!*spacing_selection.get())
    };

    let defined_length = create_signal(cx, 0.0);
    let defined_length_selection = create_signal(cx, false);

    let toggle_defined_length_selection = || {
        defined_length_selection.set(!*defined_length_selection.get())
    };

    let config_error_message = create_signal(cx, Option::<String>::None);

    let piece_list = create_signal(cx, Vec::<Rectangle>::new());
    let showcase = create_signal(cx, None);
    let prohibited_area_list = create_signal(cx, Vec::<PositionedRectangle>::new());
    
    let fit_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    let prohibited_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    let showcase_list_to_draw: &Signal<Vec<PositionedRectangle>> = create_signal(cx, Vec::new());
    let unused_rectangles_list: &Signal<Vec<Rectangle>> = create_signal(cx, Vec::new());

    let length_used = create_signal(cx, 0.0);
    let total_area = create_signal(cx, 0.0);
    let used_area = create_signal(cx, 0.0);
    let usage = create_signal(cx, 0.0);
    let max_length_to_draw = create_signal(cx, 0.0);
    let defined_width_to_draw = create_signal(cx, 0.0);

    let draw_error_message = create_signal(cx, Option::<String>::None);

    let info_error_message = create_signal(cx, Option::<String>::None);

    let get_cut_disposition_input = move || {
        spawn_local_scoped(cx, async move {
            let cut_disposition_input_result = get_cut_disposition_input().await;
            match cut_disposition_input_result {
                Ok(cut_disposition_input) => {
                    defined_width.set(cut_disposition_input.defined_width as f64);
                    max_length.set(cut_disposition_input.max_length as f64);

                    match cut_disposition_input.spacing {
                        Some(value) => {
                            spacing.set(value as f64);
                            spacing_selection.set(true)
                        },
                        None => {
                            spacing.set(0.0);
                            spacing_selection.set(false)
                        },
                    }

                    match cut_disposition_input.defined_length {
                        Some(value) => {
                            defined_length.set(value as f64);
                            defined_length_selection.set(true)
                        },
                        None => {
                            defined_length.set(0.0);
                            defined_length_selection.set(false)
                        },
                    }
                    piece_list.set(cut_disposition_input.rectangles_list);
                    showcase.set(cut_disposition_input.showcase);
                    prohibited_area_list.set(cut_disposition_input.prohibited_area_list);
                },
                Err(_error) => todo!(),
            }
        })
    };

    get_cut_disposition_input();

    let get_config_cut_disposition_input = move || {
        spawn_local_scoped(cx, async move {
            let config_cut_disposition_result = get_config_cut_disposition_input().await;
            match config_cut_disposition_result {
                Ok(config_cut_disposition) => {
                    defined_width.set(config_cut_disposition.defined_width as f64);
                    max_length.set(config_cut_disposition.max_length as f64);
                    config_error_message.set(None);
                    match config_cut_disposition.spacing {
                        Some(value) => {
                            spacing.set(value as f64);
                            spacing_selection.set(true)
                        },
                        None => {
                            spacing.set(0.0);
                            spacing_selection.set(false)
                        },
                    }

                    match config_cut_disposition.defined_length {
                        Some(value) => {
                            defined_length.set(value as f64);
                            defined_length_selection.set(true)
                        },
                        None => {
                            defined_length.set(0.0);
                            defined_length_selection.set(false)
                        },
                    }
                },
                Err(error) => {
                    config_error_message.set(Some(error.message));
                },
            }
        })
    };

    let get_cut_disposition_output_fn = move || {
        spawn_local_scoped(cx, async move {
            let cut_disposition_output_result = get_cut_disposition_output().await;
            match cut_disposition_output_result {
                Ok(cut_disposition_output) => {
                    fit_list_to_draw.set(cut_disposition_output.positioned_rectangles_list);
                    showcase_list_to_draw.set(cut_disposition_output.showcase_rectangles_located_list);
                    prohibited_list_to_draw.set(cut_disposition_output.prohibited_area_list);
                    unused_rectangles_list.set(cut_disposition_output.unused_rectangles_list);
                    length_used.set(cut_disposition_output.length_used as f64);
                    total_area.set(cut_disposition_output.total_area as f64);
                    used_area.set(cut_disposition_output.used_area as f64);
                    usage.set(cut_disposition_output.usage);
                    draw_error_message.set(None);
                    max_length_to_draw.set(cut_disposition_output.max_length as f64);
                    defined_width_to_draw.set(cut_disposition_output.defined_width as f64);
                },
                Err(error) => {
                    fit_list_to_draw.set(Vec::new());
                    showcase_list_to_draw.set(Vec::new());
                    prohibited_list_to_draw.set(Vec::new());
                    unused_rectangles_list.set(Vec::new());
                    length_used.set(0.0);
                    total_area.set(0.0);
                    used_area.set(0.0);
                    usage.set(0.0);
                    draw_error_message.set(Some(error.message));
                    max_length_to_draw.set(0.0);
                    defined_width_to_draw.set(0.0);
                },
            }
        })
    };
    
    get_cut_disposition_output_fn();

    let save_config = 
    move || {
        draw_error_message.set(None);
        info_error_message.set(None);
        config_error_message.set(None);

        spawn_local_scoped(cx, async move {
            let config = ConfigCutDispositionInput {
                spacing: get_optional_from_boolean_and_value(*spacing_selection.get(), *spacing.get() as i32),
                max_length: *max_length.get() as i32,
                defined_length: get_optional_from_boolean_and_value(*defined_length_selection.get(), *defined_length.get() as i32),
                defined_width: *defined_width.get() as i32,
            };
            let response = set_config_cut_disposition_input(config).await;
            match response {
                Ok(_) => {
                    config_error_message.set(None);
                    get_cut_disposition_output_fn();
                },
                Err(error) => {
                    config_error_message.set(Some(error.message));
                }
            }
        })
    };


    fn get_row_piece_style(piece_id: u32, fit_list: &Vec<PositionedRectangle>, not_fit_list: &Vec<Rectangle>) -> String {
        if fit_list.is_empty() && not_fit_list.is_empty() {
            "".to_string()
        } else if fit_list.iter().any(|item| item.id == piece_id) {
            "has-text-white has-background-success".to_string()
        } else {
            "has-text-white has-background-danger".to_string()
        }
    }

    let cutting_table_list: &Signal<Vec<CuttingTable>> =
        create_signal(cx, Vec::<CuttingTable>::new());

    let selected_cutting_table = create_signal(cx, String::from("0"));

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

    let fabric_list = create_signal(cx, Vec::<Fabric>::new());
    let selected_fabric = create_signal(cx, String::from("0"));

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

    let import_disposition = move || {
        draw_error_message.set(None);
        info_error_message.set(None);
        config_error_message.set(None);

        spawn_local_scoped(cx, async move {
            let import_result =
                import_disposition().await;

            match import_result {
                Ok(_value) => (),
                Err(error) => info_error_message.set(Some(error.message))
            };
            get_cut_disposition_input();
            get_cut_disposition_output_fn();
            selected_cutting_table.set(String::from("0"));
            selected_fabric.set(String::from("0"));
        })
    };

    create_effect(cx, || {
        config_error_message.set(None);
        if *selected_cutting_table.get() == "0" {
            selected_fabric.set(String::from("0"));
            max_length.set(0.0);
            defined_width.set(0.0);
        } else {
            defined_width.set(0.0);
            selected_fabric.set(String::from("0"));
            if let Some(table) = (*cutting_table_list.get())
            .iter()
            .find(|item| item.id.to_string() == *selected_cutting_table.get()) {
                max_length.set(table.length as f64);
            }
            
        }
    });

    create_effect(cx, || {
        if *selected_fabric.get() != "0" {
            config_error_message.set(None);
            if let Some(fabric) = (*fabric_list.get())
            .iter()
            .find(|item| item.id.to_string() == *selected_fabric.get()) {
                if let Some(table) = (*cutting_table_list.get())
                .iter()
                .find(|item| item.id.to_string() == *selected_cutting_table.get()) {
                    if fabric.width <= table.width {
                        defined_width.set(fabric.width as f64);
                    } else {
                        selected_fabric.set(String::from("0"));
                        defined_width.set(0.0);
                        config_error_message.set(Some("Largura do tecido deve ser menor ou igual a largura da mesa".to_string()))
                    }
                }
                
            }
            
        }
    });

    view! { cx,
        div(class="columns mx-1") {
            div(class="panel ml-2 mt-3", style="width:300px") {
                header(class="panel-heading has-background-grey-lighter level") { 
                    p(class="level-left") {"Disposição" }
                    button(class="button is-grey level-rigth", on:click=move |_| get_cut_disposition_output_fn()) { "Recarregar" }
                }
                div(class="panel-block is-flex") {
                    div(class="columns") {
                        div(class="column field") {
                            div(class="panel-block is-flex") {
                                (if (*draw_error_message.get()).is_none() {
                                    view!(cx,
                                        svg (
                                            xmlns="http://www.w3.org/2000/svg", 
                                            aria-label="Flickr", role="img", 
                                            viewBox=format!("0 0 {} {}", (defined_width_to_draw.get().to_string()), (max_length_to_draw.get().to_string())),
                                            style="border:1px solid #000000;",
                                            width="250px",
                                            length="700px",
                                            preserveAspectRatio="xMidYMid meet" 
                                        ) {
                                            //tecido
                                            rect(
                                                width=(defined_width_to_draw.get().to_string()), 
                                                height=(length_used.get().to_string()), 
                                                x="0", 
                                                y="0", 
                                                style="fill:rgb(200,200,200);stroke-width:1;stroke:rgb(0,0,0)"
                                            ) {}
                                            Keyed(
                                                iterable=fit_list_to_draw,
                                                view=|cx, item| view! { cx,
                                                    svg () {
                                                        rect(
                                                            width=(item.width.to_string()), 
                                                            height=(item.length.to_string()), 
                                                            x=(item.top_left_vertex.pos_x.to_string()), 
                                                            y=(item.top_left_vertex.pos_y.to_string()), 
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
                                                            x=(item.top_left_vertex.pos_x.to_string()), 
                                                            y=(item.top_left_vertex.pos_y.to_string()), 
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
                                                            x=(item.top_left_vertex.pos_x.to_string()), 
                                                            y=(item.top_left_vertex.pos_y.to_string()), 
                                                            style="fill:rgb(255,255,0);stroke-width:1;stroke:rgb(0,0,0)"
                                                        ) {}
                                                    }
                                                },
                                                key=|item| (item.id.to_string()),
                                            )
                                        }
                                    )
                                } else {
                                    view!(cx,
                                        p(class="has-text-danger") { (draw_error_message.get()) }
                                    )
                                })
                                
                            }
                        }
                    }
                }
            }
            div(class="column") {
                div(class="card") {
                    a(on:click=set_config_panel_active) {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Configuração"
                            }
                        }
                    }
                    div(id="collapsible-card-config", class=(match *active_panel.get() {SelectedPanel::Config => "", _ => "is-hidden"})) {
                        div(class="card-content") {
                            div(class="field") {
                                label(class="label") { "Mesa de corte"}
                                div (class="control")  {
                                    div (class="select is-fullwidth") {
                                        select(bind:value=selected_cutting_table) {
                                            option(value="0") { "Não selecionado" }
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
                                        select(bind:value=selected_fabric, disabled=(*selected_cutting_table.get()) == "0" ) {
                                            option(value="0") { "Não selecionado" }
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
                                        input(
                                            class="input", 
                                            type="number", 
                                            placeholder="Number input", 
                                            bind:valueAsNumber=defined_width,
                                            step="1",
                                            pattern="/d+",
                                            min="0",
                                            disabled=(*selected_cutting_table.get()) != "0" 
                                        ) {}
                                    }
                                }
                                div(class="column field") {
                                    label(class="label") { "Comprimento máximo (mm)"}
                                    div (class="control")  {
                                        input(
                                            class="input", 
                                            type="number", 
                                            placeholder="Number input", 
                                            bind:valueAsNumber=max_length,
                                            step="1",
                                            pattern="/d+",
                                            min="0",
                                            disabled=(*selected_cutting_table.get()) != "0" 
                                        ) {}
                                    }
                                }
                            }
                            div(class="columns") {
                                div(class="column field") {
                                    label(class="label") { "Espaçamento(mm)" }
                                    div (class="level")  {
                                        div(class="level-left") {
                                            input(
                                                class="toggle",
                                                type="checkbox",
                                                on:input=move |_| toggle_spacing_selection(),
                                                bind:checked=spacing_selection
                                            )
                                        }
                                        div(class="level-item") {
                                            div (class="control")  {
                                                input(
                                                    class="input", 
                                                    type="number", 
                                                    placeholder="Number input", 
                                                    bind:valueAsNumber=spacing, 
                                                    disabled = !*spacing_selection.get(),
                                                    step="1",
                                                    pattern="/d+",
                                                    min="0"
                                                ) {}
                                            }
                                        }
                                    }
                                }
                                div(class="column field") {
                                    label(class="label") { 
                                        "Comprimento definido (mm)"
                                    }
                                    div (class="level")  {
                                        div(class="level-left") {
                                            input(
                                                class="toggle",
                                                type="checkbox",
                                                on:input=move |_| toggle_defined_length_selection(),
                                                bind:checked=defined_length_selection,
                                            )
                                        }
                                        div(class="level-item") {
                                            div (class="control")  {
                                                input(
                                                    class="input", 
                                                    type="number", 
                                                    placeholder="Number input",
                                                    bind:valueAsNumber=defined_length, 
                                                    disabled = !*defined_length_selection.get(),
                                                    step="1",
                                                    pattern="/d+",
                                                    min="0"
                                                ) {}
                                            }
                                        }
                                    }
                                }
                            }
                            div(class="column field") {
                                p(class="has-text-danger") { (config_error_message.get()) }
                            }
                            div(class="level") {
                                div(class="level-item") {
                                    button(class="button is-success mr-5", on:click=move |_| save_config()) { "Salvar" }
                                    button(class="button is-warning", on:click=move |_| get_config_cut_disposition_input()) { "Cancelar" }
                                }
                            }
                        }
                    }
                }
                div(class="card") {
                    a(on:click=set_pieces_panel_active) {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Peças"
                            }
                        }
                    }
                    div(id="collapsible-card-pieces", class=(match *active_panel.get() {SelectedPanel::Pieces => "", _ => "is-hidden"})) {
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
                                            a(class="button is-responsive is-success", href="/piece-item") { "Nova" }
                                        }
                                    }
                                }
                                tbody {
                                    (if let Some(some_showcase) = (*showcase.get()).clone() {
                                        view!(cx, 
                                            tr(class="has-background-warning") {
                                                td (style="vertical-align:middle;") { (some_showcase.id.clone()) }
                                                td (style="vertical-align:middle;") { (some_showcase.width.clone())  }
                                                td (style="vertical-align:middle;") { (some_showcase.length.clone()) }
                                                td (style="vertical-align:middle;") {  }
                                                td (style="vertical-align:middle;") {  }
                                                td (style="vertical-align:middle;") {
                                                    a(class="button is-responsive is-info", href=(format!("/edit-piece-item/2/{}", some_showcase.id ))) { "Editar" }
                                                }
                                            }
                                        )
                                    } else {
                                        view!(cx, )
                                    })
                                    
                                    Keyed(
                                        iterable=piece_list,
                                        view=move |cx, item| view! { cx,
                                            tr(class=format!("{}", get_row_piece_style(item.id, &fit_list_to_draw.get(), &unused_rectangles_list.get()))) {
                                                td (style="vertical-align:middle;") { (item.id.clone()) }
                                                td (style="vertical-align:middle;") { (item.width.clone())  }
                                                td (style="vertical-align:middle;") { (item.length.clone()) }
                                                td (style="vertical-align:middle;") {  }
                                                td (style="vertical-align:middle;") {  }
                                                td (style="vertical-align:middle;") {
                                                    a(class="button is-responsive is-info", href=(format!("/edit-piece-item/1/{}", item.id ))) { "Editar" }
                                                }
                                            }
                                        },
                                        key=|item| item.id,
                                    )
                                    Keyed(
                                        iterable=prohibited_area_list,
                                        view=move |cx, item| view! { cx,
                                            tr(class="has-text-white has-background-black") {
                                                td (style="vertical-align:middle;") { (item.id.clone()) }
                                                td (style="vertical-align:middle;") { (item.width.clone())  }
                                                td (style="vertical-align:middle;") { (item.length.clone()) }
                                                td (style="vertical-align:middle;") { (item.top_left_vertex.pos_x.clone()) }
                                                td (style="vertical-align:middle;") { (item.top_left_vertex.pos_y.clone()) }
                                                td (style="vertical-align:middle;") {
                                                    a(class="button is-responsive is-info", href=(format!("/edit-piece-item/3/{}", item.id ))) { "Editar" }
                                                }
                                            }
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


                div(class="card") {
                    a(on:click=set_info_panel_active) {
                        header(class="card-header has-background-grey-lighter") {
                            p(class="card-header-title") {
                                "Informações"
                            }
                        }
                    }
                    div(id="collapsible-card-pieces", class=(match *active_panel.get() {SelectedPanel::Info => "", _ => "is-hidden"})) {
                        div(class="card-content") {
                            div(class="columns") {
                                div(class="column field") {
                                    label(class="label") {
                                        "Área total(m³)"
                                    }
                                    p {
                                        (format!("{}", (total_area.get().abs() / 1_000_000_f64)))
                                    }
                                }
                                div(class="column field") {
                                    label(class="label") {
                                        "Área aproveitada(m³)"
                                    }
                                    p {
                                        (format!("{}", (used_area.get().abs() / 1_000_000_f64)))
                                    }
                                }
                                div(class="column field") {
                                    label(class="label") {
                                        "Aproveitamento(%)"
                                    }
                                    p {
                                        (format!("{:.2}", usage.get()))
                                    }
                                }
                                div(class="column field") {
                                    label(class="label") {
                                        "Comprimento utilizado(mm)"
                                    }
                                    p {
                                        (length_used.get())
                                    }
                                }
                            }
                            div(class="columns") {
                                div(class="column level") {
                                    a(class="button is-responsive is-info level-item", href="/generate-g-code") { "Gerar G-code" }
                                }
                                div(class="column level") {
                                    a(class="button is-responsive is-info level-item", href="/export-disposition") { "Exportar Disposição" }
                                }
                                div(class="column level") {
                                    a(class="button is-responsive is-info level-item", on:click=move |_| import_disposition()) { "Importar Disposição" }
                                }
                            }
                            div(class="columns") {
                                div(class="column level") {
                                    p(class="has-text-danger") { (info_error_message.get()) }
                                }
                            }
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