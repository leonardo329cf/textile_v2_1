use std::time::Duration;

use tauri::State;
use tokio::time::sleep;

use crate::{
    db::db_connection::DbConnection,
    models::{
        app_error::{AppError, DEFAULT_ERROR_CODE},
        cutting_table::{self, CuttingTable, CuttingTableCreate},
    },
    services::cutting_table_service,
};

#[tauri::command]
pub async fn get_cutting_table(
    id: i32,
    db_state: State<'_, DbConnection>,
) -> Result<CuttingTable, AppError> {
    let result = cutting_table::get(id, &db_state.db).await;
    match result {
        Ok(table) => Ok(table),
        Err(_error) => Err(AppError::new(
            DEFAULT_ERROR_CODE,
            format!("Falha ao buscar Mesa: {}", id).as_str(),
        )),
    }
}

#[tauri::command]
pub async fn get_all_cutting_table(
    db_state: State<'_, DbConnection>,
) -> Result<Vec<CuttingTable>, AppError> {
    // Wait for a milisec because it was returning before the table was updated
    sleep(Duration::from_millis(1)).await;
    let result = cutting_table::get_all(&db_state.db).await;
    match result {
        Ok(table) => Ok(table),
        Err(_error) => Err(AppError::new(
            DEFAULT_ERROR_CODE,
            "Falha ao buscar lista de Mesa",
        )),
    }
}

#[tauri::command]
pub async fn delete_cutting_table(
    id: i32,
    db_state: State<'_, DbConnection>,
) -> Result<CuttingTable, AppError> {
    let result = cutting_table::delete(id, &db_state.db).await;
    match result {
        Ok(table) => Ok(table),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao remover Mesa")),
    }
}

#[tauri::command]
pub async fn create_cutting_table(
    table: CuttingTableCreate,
    db_state: State<'_, DbConnection>,
) -> Result<CuttingTable, AppError> {
    cutting_table_service::create(table, &db_state.db).await
}

#[tauri::command]
pub async fn update_cutting_table(
    table: CuttingTable,
    db_state: State<'_, DbConnection>,
) -> Result<CuttingTable, AppError> {
    cutting_table_service::update(table, &db_state.db).await
}
