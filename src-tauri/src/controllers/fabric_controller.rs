use tauri::State;

use crate::{db::db_connection::DbConnection, models::{fabric::{self, Fabric, FabricCreate}, app_error::{AppError, DEFAULT_ERROR_CODE}}, services::fabric_service}; 

#[tauri::command]
pub async fn get_fabric(id: i32, db_state: State<'_, DbConnection>) -> Result<Fabric, AppError> {
    let result = fabric::get(id, &db_state.db).await;
    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, format!("Falha ao buscar Tecido: {}", id).as_str()))
    }
}

#[tauri::command]
pub async fn get_all_fabric(db_state: State<'_, DbConnection>) -> Result<Vec<Fabric>, AppError> {
    let result = fabric::get_all(&db_state.db).await;
    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao buscar lista de Tecido")),
    }
}

#[tauri::command]
pub async fn delete_fabric(id: i32, db_state: State<'_, DbConnection>) -> Result<Fabric, AppError> {
    let result = fabric::delete(id, &db_state.db).await;
    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao remover Tecido")),
    }
}

#[tauri::command]
pub async fn create_fabric(fabric: FabricCreate, db_state: State<'_, DbConnection>) -> Result<Fabric, AppError> {
    fabric_service::create(fabric, &db_state.db).await
}

#[tauri::command]
pub async fn update_fabric(fabric: Fabric, db_state: State<'_, DbConnection>) -> Result<Fabric, AppError> {
    fabric_service::update(fabric, &db_state.db).await
}