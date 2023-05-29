use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::app::{invoke, models::app_error::AppError, log};

#[derive(Serialize, Deserialize)]
struct AboutArgs<> {
}

pub async fn get_about() -> Result<String, AppError> {
    let value = invoke("get_about", to_value(&AboutArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<String>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(_) => {
                    log("Failed to convert an successful result for get_about");
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar o arquivo".to_owned(), 
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
                Err(_) => {
                    log("Failed to convert an successful result for get_about");
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar o arquivo".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
} 