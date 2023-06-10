use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::app::{invoke, models::{app_error::AppError, fabric::Fabric}, log};

#[derive(Serialize, Deserialize)]
struct FabricArgs<> {
}

pub async fn get_all_fabric() -> Result<Vec<Fabric>, AppError> {
    let value = invoke("get_all_fabric", to_value(&FabricArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Vec<Fabric>>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(_) => {
                    log("Failed to convert an successful result for get_all_fabric");
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar lista de tecidos".to_owned(), 
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
                    log("Failed to convert an AppError result for get_all_fabric");
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar lista de tecidos".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
} 