use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use crate::app::{models::{cut_disposition::CutDispositionInput, app_error::AppError}, invoke, log};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfigCutDispositionInput {
    pub spacing: Option<i32>,
    pub max_length: i32,
    pub defined_length: Option<i32>,
    pub defined_width: i32,
}

#[derive(Serialize, Deserialize)]
struct ConfigConfigCutDispositionInputArgs<> {
    config: ConfigCutDispositionInput
}

pub async fn set_config_cut_disposition_input(config: ConfigCutDispositionInput) -> Result<CutDispositionInput, AppError> {
    let value = invoke("set_config_cut_disposition_input", to_value(&ConfigConfigCutDispositionInputArgs { config }).unwrap()).await;
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