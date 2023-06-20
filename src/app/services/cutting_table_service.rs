use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::app::{
    invoke, log,
    models::{
        app_error::AppError,
        cutting_table::{CuttingTable, CuttingTableCreate},
    },
};

#[derive(Serialize, Deserialize)]
struct CuttingTableNoArgs {}

pub async fn get_all_cutting_table() -> Result<Vec<CuttingTable>, AppError> {
    let value = invoke(
        "get_all_cutting_table",
        to_value(&CuttingTableNoArgs {}).unwrap(),
    )
    .await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<Vec<CuttingTable>>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao buscar lista de mesas".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao buscar lista de mesas".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CuttingTableWithIdArgs {
    id: i32,
}

pub async fn get_cutting_table_by_id(id: i32) -> Result<CuttingTable, AppError> {
    let value = invoke(
        "get_cutting_table",
        to_value(&CuttingTableWithIdArgs { id }).unwrap(),
    )
    .await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<CuttingTable>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao buscar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao buscar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
    }
}

pub async fn delete_cutting_table(id: i32) -> Result<CuttingTable, AppError> {
    let value = invoke(
        "delete_cutting_table",
        to_value(&CuttingTableWithIdArgs { id }).unwrap(),
    )
    .await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<CuttingTable>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao deletar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao deletar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CuttingTableCreateArgs {
    cuttingTable: CuttingTableCreate,
}

pub async fn create_cutting_table(
    cutting_table: CuttingTableCreate,
) -> Result<CuttingTable, AppError> {
    let value = invoke(
        "create_cutting_table",
        to_value(&CuttingTableCreateArgs {
            cuttingTable: cutting_table,
        })
        .unwrap(),
    )
    .await;
    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<CuttingTable>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao criar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao criar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]

struct CuttingTableUpdateArgs {
    cuttingTable: CuttingTable,
}

pub async fn update_cutting_table(cutting_table: CuttingTable) -> Result<CuttingTable, AppError> {
    let value = invoke(
        "update_cutting_table",
        to_value(&CuttingTableUpdateArgs {
            cuttingTable: cutting_table,
        })
        .unwrap(),
    )
    .await;

    match value {
        Ok(ok_js_value) => {
            let a = serde_wasm_bindgen::from_value::<CuttingTable>(ok_js_value);
            match a {
                Ok(a) => Ok(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao atualizar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
        Err(err_js_value) => {
            let a = serde_wasm_bindgen::from_value::<AppError>(err_js_value);
            match a {
                Ok(a) => Err(a),
                Err(error) => {
                    log(error.to_string().as_str());
                    Err(AppError {
                        status: 1,
                        message: "Falha ao atualizar mesa".to_owned(),
                        timestamp: 1,
                    })
                }
            }
        }
    }
}
