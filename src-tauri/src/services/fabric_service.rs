use sqlx::{Pool, Sqlite};

use crate::{models::{fabric::{self, Fabric, FabricCreate}, app_error::{AppError, DEFAULT_ERROR_CODE}}};

pub async fn create(fabric: FabricCreate, poll: &Pool<Sqlite>) -> Result<Fabric, AppError> {
    let trimmed_fabric = trim_create_fabric_create_fields(fabric).clone();

    verify_create_fabric_create_fields(&trimmed_fabric)?;

    let result = fabric::create(trimmed_fabric, poll).await;

    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao salvar Tecido")),
    }
}

fn trim_create_fabric_create_fields(fabric: FabricCreate) -> FabricCreate {
    let mut new_fabric = fabric;
    new_fabric.name = new_fabric.name.trim().to_owned();
    new_fabric.manufacturer = new_fabric.manufacturer.trim().to_owned();
    new_fabric.code = new_fabric.code.trim().to_owned();

    new_fabric
}

fn verify_create_fabric_create_fields(fabric: &FabricCreate) -> Result<(), AppError> {
    if fabric.name.trim().is_empty() {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo obrigatório: nome"))
    }
    if fabric.width <= 0 {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo deve ser maior que zero: largura"))
    }
    Ok(())
}

pub async fn update(fabric: Fabric, poll: &Pool<Sqlite>) -> Result<Fabric, AppError> {
    let trimmed_fabric = trim_create_fabric_fields(fabric).clone();

    verify_create_fabric_fields(&trimmed_fabric)?;

    let result = fabric::update(trimmed_fabric, poll).await;

    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao salvar Tecido")),
    }
}

fn trim_create_fabric_fields(fabric: Fabric) -> Fabric {
    let mut new_fabric = fabric;
    new_fabric.name = new_fabric.name.trim().to_owned();
    new_fabric.manufacturer = new_fabric.manufacturer.trim().to_owned();
    new_fabric.code = new_fabric.code.trim().to_owned();

    new_fabric
}

fn verify_create_fabric_fields(fabric: &Fabric) -> Result<(), AppError> {
    if fabric.name.trim().is_empty() {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo obrigatório: nome"))
    }
    if fabric.width <= 0 {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo deve ser maior que zero: largura"))
    }
    Ok(())
}