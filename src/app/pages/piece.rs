use std::fmt::Display;

use serde::{Serialize, Deserialize};
use sycamore::{prelude::*, futures::spawn_local_scoped};
use sycamore_router::navigate;

use crate::app::{models::{piece::RectangleType, cut_disposition::{Rectangle, PositionedRectangle, Vertex}, app_error::AppError}, services::cut_disposition_service::{create_piece, get_piece_by_id, get_showcase, get_prohibited_area_by_id, edit_piece, remove_prohibited_area_by_id, remove_piece, remove_showcase}, log};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PieceType {
    Piece,
    Showcase,
    ProhibitedArea
}
impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PieceType::Piece => write!(f, "Peça"),
            PieceType::Showcase => write!(f, "Mostruário"),
            PieceType::ProhibitedArea => write!(f, "Área Proibida"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PieceOption {
    id: String,
    text: String,
    piece_type: PieceType
}

#[component]
pub fn PieceItemPage< G: Html>(cx: Scope<'_>) -> View<G> {
    let id = create_signal(cx, 0.0);
    let width = create_signal(cx, 0.0);
    let length = create_signal(cx, 0.0);
    let pos_x = create_signal(cx, 0.0);
    let pos_y = create_signal(cx, 0.0);

    let piece_type = create_signal(cx, "1".to_string());

    let error_message = create_signal(cx, String::new());

    let piece_options: Vec<PieceOption>  = vec![
        PieceOption { 
            id: "1".to_string(), 
            text: "Peça".to_string(), 
            piece_type: PieceType::Piece 
        },
        PieceOption { 
            id: "2".to_string(), 
            text: "Mostruário".to_string(), 
            piece_type: PieceType::Showcase 
        },
        PieceOption { 
            id: "3".to_string(), 
            text: "Área Proibida".to_string(), 
            piece_type: PieceType::ProhibitedArea 
        },
    ];

    let piece_options_list = create_signal(cx, piece_options);

    let save_item =  move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = *id.get().as_ref() as u32;
            let param_width = *width.get().as_ref() as i32;
            let param_length = *length.get().as_ref() as i32;
            let param_pos_x = *pos_x.get().as_ref() as i32;
            let param_pos_y = *pos_y.get().as_ref() as i32;

            log((*piece_type.get()).as_str());

            let response = match (*piece_type.get()).as_str() {
                "1" => {
                    create_piece(
                        RectangleType::Piece(
                            Rectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                            }
                        )
                    ).await
                },
                "2" => {
                    create_piece(
                        RectangleType::Showcase(
                            Rectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                            }
                        )
                    ).await
                },
                "3" => {
                    create_piece(
                        RectangleType::ProhibitedArea(
                            PositionedRectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                                top_left_vertex: Vertex {
                                    pos_x: param_pos_x,
                                    pos_y: param_pos_y,
                                }
                            }
                        )
                    ).await
                },
                _ => {
                    Err(AppError {
                        status:1,
                        message: "Tipo de peça invalido".to_string(),
                        timestamp: 1
                    })
                }
            };

            match response {
                Ok(()) => {
                    navigate("/fabric-cut")
                },
                Err(e) => error_message.set(e.message),
            }
        });
    };

    view! { cx,
        div(class="modal is-active") {
            div(class="modal-background") {}
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title level-left") { "Criar de Peça" } 
                    div(class="level-item level-right") {
                        a(class="button is-medium is-warning", href="/fabric-cut") { "Cancelar" }
                    }
                }
                section(class="modal-card-body") {
                    div(class="field") {
                        label(class="label") { "Largura (mm)" }
                        div (class="control")  {
                            input(
                                class="input", 
                                type="number", 
                                placeholder="Number input", 
                                bind:valueAsNumber=width, 
                                step="1",
                                pattern="/d+",
                                min="0"
                            ) {}
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Comprimento (mm)" }
                        div (class="control")  {
                            input(
                                class="input", 
                                type="number", 
                                placeholder="Number input", 
                                bind:valueAsNumber=length, 
                                step="1",
                                pattern="/d+",
                                min="0"
                            ) {}
                        }
                    }
                    (if (*piece_type.get()).as_str() == "3" {
                        view!(cx, 
                            div(class="field") {
                                label(class="label") { "Posição X (mm)" }
                                div (class="control")  {
                                    input(
                                        class="input", 
                                        type="number", 
                                        placeholder="Number input", 
                                        bind:valueAsNumber=pos_x, 
                                        step="1",
                                        pattern="/d+",
                                        min="0"
                                    ) {}
                                }
                            }
                            div(class="field") {
                                label(class="label") { "Posição Y (mm)" }
                                div (class="control")  {
                                    input(
                                        class="input", 
                                        type="number", 
                                        placeholder="Number input", 
                                        bind:valueAsNumber=pos_y, 
                                        step="1",
                                        pattern="/d+",
                                        min="0"
                                    ) {}
                                }
                            }
                        )
                    } else {
                        view!(cx, )
                    })
                    div(class="field") {
                        label(class="label") { "Tipo de peça"}
                        div (class="control")  {
                            div (class="select is-fullwidth") {
                                select(bind:value=piece_type) {
                                    Keyed(
                                        iterable=piece_options_list,
                                        view=move |cx, item| view! { cx,
                                            PieceOption(option = item) {}
                                        },
                                        key=|item| item.id.clone(),
                                    )

                                }
                            }
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
                    }
                }
            }
        }
    }
}

#[derive(Props)]
pub struct EditPieceItemProps<> {
    piece_type_id: i32,
    id: u32,
}

#[component]
pub fn EditPieceItemPage< G: Html>(cx: Scope<'_>, props: EditPieceItemProps) -> View<G> {
    let id = create_signal(cx, 0.0);
    let width = create_signal(cx, 0.0);
    let length = create_signal(cx, 0.0);
    let pos_x = create_signal(cx, 0.0);
    let pos_y = create_signal(cx, 0.0);

    let piece_type = create_signal(cx, props.piece_type_id.to_string());

    let error_message = create_signal(cx, String::new());

    let piece_options: Vec<PieceOption>  = vec![
        PieceOption { 
            id: "1".to_string(), 
            text: "Peça".to_string(), 
            piece_type: PieceType::Piece 
        },
        PieceOption { 
            id: "2".to_string(), 
            text: "Mostruário".to_string(), 
            piece_type: PieceType::Showcase 
        },
        PieceOption { 
            id: "3".to_string(), 
            text: "Área Proibida".to_string(), 
            piece_type: PieceType::ProhibitedArea 
        },
    ];

    let piece_options_list = create_signal(cx, piece_options);

    
    let get_data = move || {
        spawn_local_scoped(cx, async move {
            match (*piece_type.get()).as_str() {
                "1" => {
                    let item = get_piece_by_id(props.id).await;
                    match item {
                        Ok(piece) => {
                            id.set(piece.id as f64);
                            width.set(piece.width as f64);
                            length.set(piece.length as f64);
                        },
                        Err(error) => {
                            error_message.set(error.message);
                        },
                    }
                },
                "2" => {
                    let item = get_showcase().await;
                    match item {
                        Ok(piece_option) => {
                            match piece_option {
                                Some(piece) => {
                                    id.set(piece.id as f64);
                                    width.set(piece.width as f64);
                                    length.set(piece.length as f64);
                                },
                                None => {
                                    error_message.set("Mostruário não existe".to_owned());
                                }
                            }
                        },
                        Err(error) => {
                            error_message.set(error.message);
                        },
                    }
                },
                "3" => {
                    let item: Result<PositionedRectangle, AppError> = get_prohibited_area_by_id(props.id).await;
                    match item {
                        Ok(piece) => {
                            id.set(piece.id as f64);
                            width.set(piece.width as f64);
                            length.set(piece.length as f64);
                            pos_x.set(piece.top_left_vertex.pos_x as f64);
                            pos_y.set(piece.top_left_vertex.pos_y as f64);
                        },
                        Err(error) => {
                            error_message.set(error.message);
                        },
                    }
                },
                _ => {
                    error_message.set("Tipo de peça inválido".to_string());
                }
            }
        })
    };

    get_data();

    let save_item =  move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = *id.get().as_ref() as u32;
            let param_width = *width.get().as_ref() as i32;
            let param_length = *length.get().as_ref() as i32;
            let param_pos_x = *pos_x.get().as_ref() as i32;
            let param_pos_y = *pos_y.get().as_ref() as i32;

            log((*piece_type.get()).as_str());

            let response = match (*piece_type.get()).as_str() {
                "1" => {
                    edit_piece(
                        RectangleType::Piece(
                            Rectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                            }
                        )
                    ).await
                },
                "2" => {
                    edit_piece(
                        RectangleType::Showcase(
                            Rectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                            }
                        )
                    ).await
                },
                "3" => {
                    edit_piece(
                        RectangleType::ProhibitedArea(
                            PositionedRectangle {
                                id: param_id,
                                width: param_width,
                                length: param_length,
                                top_left_vertex: Vertex {
                                    pos_x: param_pos_x,
                                    pos_y: param_pos_y,
                                }
                            }
                        )
                    ).await
                },
                _ => {
                    Err(AppError {
                        status:1,
                        message: "Tipo de peça invalido".to_string(),
                        timestamp: 1
                    })
                }
            };

            match response {
                Ok(()) => {
                    navigate("/fabric-cut")
                },
                Err(e) => error_message.set(e.message),
            }
        });
    };

    let remove_item =  move |_| {
        spawn_local_scoped(cx, async move {
            let param_id = *id.get().as_ref() as u32;

            log((*piece_type.get()).as_str());

            let response = match (*piece_type.get()).as_str() {
                "1" => {
                    remove_piece(param_id).await
                },
                "2" => {
                    remove_showcase(param_id).await
                },
                "3" => {
                    remove_prohibited_area_by_id(param_id).await
                },
                _ => {
                    Err(AppError {
                        status:1,
                        message: "Tipo de peça invalido".to_string(),
                        timestamp: 1
                    })
                }
            };

            match response {
                Ok(()) => {
                    navigate("/fabric-cut")
                },
                Err(e) => error_message.set(e.message),
            }
        });
    };

    view! { cx,
        div(class="modal is-active") {
            div(class="modal-background") {}
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title level-left") { "Editar de Peça" } 
                    div(class="level-item level-right") {
                        a(class="button is-medium is-warning", href="/fabric-cut") { "Cancelar" }
                    }
                }
                section(class="modal-card-body") {
                    div(class="field") {
                        label(class="label") { "Id" }
                        label(class="label") { (id.get()) }
                    }
                    div(class="field") {
                        label(class="label") { "Largura (mm)" }
                        div (class="control")  {
                            input(
                                class="input", 
                                type="number", 
                                placeholder="Number input", 
                                bind:valueAsNumber=width, 
                                step="1",
                                pattern="/d+",
                                min="0"
                            ) {}
                        }
                    }
                    div(class="field") {
                        label(class="label") { "Comprimento (mm)" }
                        div (class="control")  {
                            input(
                                class="input", 
                                type="number", 
                                placeholder="Number input", 
                                bind:valueAsNumber=length, 
                                step="1",
                                pattern="/d+",
                                min="0"
                            ) {}
                        }
                    }
                    (if (*piece_type.get()).as_str() == "3" {
                        view!(cx, 
                            div(class="field") {
                                label(class="label") { "Posição X (mm)" }
                                div (class="control")  {
                                    input(
                                        class="input", 
                                        type="number", 
                                        placeholder="Number input", 
                                        bind:valueAsNumber=pos_x, 
                                        step="1",
                                        pattern="/d+",
                                        min="0"
                                    ) {}
                                }
                            }
                            div(class="field") {
                                label(class="label") { "Posição Y (mm)" }
                                div (class="control")  {
                                    input(
                                        class="input", 
                                        type="number", 
                                        placeholder="Number input", 
                                        bind:valueAsNumber=pos_y, 
                                        step="1",
                                        pattern="/d+",
                                        min="0"
                                    ) {}
                                }
                            }
                        )
                    } else {
                        view!(cx, )
                    })
                    div(class="field") {
                        label(class="label") { "Tipo de peça"}
                        div (class="control")  {
                            div (class="select is-fullwidth") {
                                select(bind:value=piece_type, disabled=true) {
                                    Keyed(
                                        iterable=piece_options_list,
                                        view=move |cx, item| view! { cx,
                                            PieceOption(option = item) {}
                                        },
                                        key=|item| item.id.clone(),
                                    )

                                }
                            }
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
                        div(class="level-left") {
                            button(class="button is-medium is-danger", on:click=remove_item) { "Remover" }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Props)]
pub struct PieceOptionProps {
    option: PieceOption,
}

#[component]
pub fn PieceOption<G: Html>(cx: Scope, props: PieceOptionProps) -> View<G> {
    let item = create_ref(cx, props.option);
    let id = &item.id;
    view! { cx,
        option(value=id) { (item.text) }
    }
}