use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::app::{models::app_error::AppError, invoke, log};


#[derive(Serialize, Deserialize)]
pub struct ExportDisposition {
    fileName: String, 
}

pub async fn export_disposition(
    file_name: String, 
) -> Result<String, AppError> { 
    let value = invoke("export_disposition", to_value(&ExportDisposition {fileName: file_name}).unwrap()).await;
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
                            message: "Falha exportar disposição de peças".to_owned(), 
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
                            message: "Falha exportar disposição de peças".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}