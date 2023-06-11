use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

use crate::app::{invoke, models::{app_error::AppError, fabric::{Fabric, FabricCreate}}, log};

#[derive(Serialize, Deserialize)]
struct FabricNoArgs<> {
}

pub async fn get_all_fabric() -> Result<Vec<Fabric>, AppError> {
    let value = invoke("get_all_fabric", to_value(&FabricNoArgs { }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Vec<Fabric>>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
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
                Err(error) => {
                    log(error.to_string().as_str());
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

#[derive(Serialize, Deserialize)]
struct FabricWithIdArgs<> {
    id: i32
}


pub async fn get_fabric_by_id(id: i32) -> Result<Fabric, AppError> {
    let value = invoke("get_fabric", to_value( &FabricWithIdArgs { id }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Fabric>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao buscar tecido".to_owned(), 
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
                            message: "Falha ao buscar tecido".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

pub async fn delete_fabric(id: i32) -> Result<Fabric, AppError> {
    let value = invoke("delete_fabric", to_value(&FabricWithIdArgs { id }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Fabric>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao deletar tecido".to_owned(), 
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
                            message: "Falha ao deletar tecido".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FabricCreateArgs<> {
    fabric: FabricCreate
}

pub async fn create_fabric(fabric: FabricCreate) -> Result<Fabric, AppError> {
    let value = invoke("create_fabric", to_value(&FabricCreateArgs { fabric }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Fabric>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao criar tecido".to_owned(), 
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
                            message:  "Falha ao criar tecido".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]

struct FabricUpdateArgs<> {
    fabric: Fabric
}

pub async fn update_fabric(fabric: Fabric) -> Result<Fabric, AppError> {
    let value = invoke("update_fabric", to_value(&FabricUpdateArgs{ fabric }).unwrap()).await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Fabric>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(
                        AppError {
                            status:1, 
                            message: "Falha ao atualizar tecido".to_owned(), 
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
                            message: "Falha ao atualizar tecido".to_owned(), 
                            timestamp: 1
                        }
                    )
                }
            }
        }
    }
}