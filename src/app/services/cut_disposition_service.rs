use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use crate::app::{models::{cut_disposition::{CutDispositionInput, ConfigCutDispositionInput, Rectangle, PositionedRectangle}, app_error::AppError, piece::RectangleType}, invoke, log};

#[derive(Serialize, Deserialize)]
struct NoArgs<> {
}

pub async fn get_cut_disposition_input() -> Result<CutDispositionInput, AppError> {
    let value = invoke("get_cut_disposition_input", to_value(&NoArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<CutDispositionInput>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar disposição de cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar disposição de cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

pub async fn get_config_cut_disposition_input() -> Result<ConfigCutDispositionInput, AppError> {
    let value = invoke("get_config_cut_disposition_input", to_value(&NoArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<ConfigCutDispositionInput>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar disposição de cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar disposição de cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigConfigCutDispositionInputArgs<> {
    config: ConfigCutDispositionInput
}

pub async fn set_config_cut_disposition_input(config: ConfigCutDispositionInput) -> Result<ConfigCutDispositionInput, AppError> {
    let value = invoke("set_config_cut_disposition_input", to_value(&ConfigConfigCutDispositionInputArgs { config }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<ConfigCutDispositionInput>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha mostrar resposta".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao configurar cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreatePieceArgs<> {
    piece: RectangleType
}

pub async fn create_piece(piece: RectangleType) -> Result<(), AppError> {
    let value = invoke("create_piece", to_value(&CreatePieceArgs { piece }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            Ok(())
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao configurar cortes".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct GetWithIdArgs<> {
    id: u32
}

pub async fn get_piece_by_id(id: u32) -> Result<Rectangle, AppError> {
    let value = invoke("get_piece", to_value(&GetWithIdArgs { id }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Rectangle>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar Peça".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar peça".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

pub async fn get_showcase() -> Result<Option<Rectangle>, AppError> {
    let value = invoke("get_showcase", to_value(&NoArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Option<Rectangle>>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar Mostruário".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar Mostruário".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

pub async fn get_prohibited_area_by_id(id: u32) -> Result<PositionedRectangle, AppError> {
    let value = invoke("get_prohibited_area", to_value(&GetWithIdArgs { id }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<PositionedRectangle>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar Área Proibida".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        },
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar Área Proibida".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}
