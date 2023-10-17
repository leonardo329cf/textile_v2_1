use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::app::{models::app_error::AppError, invoke, log};

#[derive(Serialize, Deserialize)]
pub struct GenerateGCodeArgs {
    fileName: String, 
    pullTextile: bool,
}

pub async fn generate_g_code_file(
    file_name: String, 
    pull_textile: bool
) -> Result<String, AppError> { 
    let value = invoke("generate_g_code", to_value(&GenerateGCodeArgs {fileName: file_name, pullTextile: pull_textile}).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<String>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao gerar código g".to_owned(), 
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
                            message: "Falha ao gerar código g".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}