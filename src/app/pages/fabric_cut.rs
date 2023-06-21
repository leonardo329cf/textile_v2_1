
use std::clone;

use serde_json::json;
use sycamore::{prelude::*, futures::spawn_local_scoped};

use crate::app::{services::cut_disposition_service::{get_cut_disposition_input, set_config_cut_disposition_input, get_config_cut_disposition_input}, utils::utils::get_optional_from_boolean_and_value, log, models::cut_disposition::{ConfigCutDispositionInput, Rectangle, PositionedRectangle}};

enum SelectedPanel {
    Config,
    Pieces,
}

#[component]
pub fn FabricCutPage<G: Html>(cx: Scope<'_>) -> View<G> {

    let active_panel = create_signal(cx, SelectedPanel::Config);

    let set_config_panel_active = |_| active_panel.set(SelectedPanel::Config);

    let set_pieces_panel_active = |_| active_panel.set(SelectedPanel::Pieces);

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

    let piece_list = create_signal(cx, Vec::<Rectangle>::new());
    let showcase = create_signal(cx, None);
    let prohibited_area_list = create_signal(cx, Vec::<PositionedRectangle>::new());

    let save_config = 
    move || {
        spawn_local_scoped(cx, async move {
            let config = ConfigCutDispositionInput {
                spacing: get_optional_from_boolean_and_value(*spacing_selection.get(), *spacing.get() as i32),
                max_length: *max_length.get() as i32,
                defined_length: get_optional_from_boolean_and_value(*defined_length_selection.get(), *defined_length.get() as i32),
                defined_width: *defined_width.get() as i32,
            };
            log(serde_json::to_string(&config).unwrap_or("default".to_owned()).as_str());
            log(serde_json::to_string(&set_config_cut_disposition_input(config).await.expect("falhou")).unwrap_or("failed to save config".to_owned()).as_str());
        })
    };

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
                Err(_error) => todo!(),
            }
        })
    };

    view! { cx,
        div(class="columns mx-1") {
            div(class="panel ml-2 mt-3", style="width:300px") {
                header(class="panel-heading has-background-grey-lighter") { "Disposição" }
                div(class="panel-block is-flex") {
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
                                            min="0"
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
                                            min="0"
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
                                            tr(class="") {
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

            }
        }
    }
}
