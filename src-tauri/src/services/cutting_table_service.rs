use sqlx::{Pool, Sqlite};

use crate::models::{
    app_error::{AppError, DEFAULT_ERROR_CODE},
    cutting_table::{self, CuttingTable, CuttingTableCreate},
};

pub async fn create(
    cutting_table: CuttingTableCreate,
    poll: &Pool<Sqlite>,
) -> Result<CuttingTable, AppError> {
    let trimmed_cutting_table = trim_create_cutting_table_create_fields(cutting_table).clone();

    verify_create_cutting_table_create_fields(&trimmed_cutting_table)?;

    let result = cutting_table::create(trimmed_cutting_table, poll).await;

    match result {
        Ok(fab) => Ok(fab),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao salvar Mesa")),
    }
}

fn trim_create_cutting_table_create_fields(
    cutting_table: CuttingTableCreate,
) -> CuttingTableCreate {
    let mut new_cutting_table = cutting_table;
    new_cutting_table.name = new_cutting_table.name.trim().to_owned();

    new_cutting_table
}

fn verify_create_cutting_table_create_fields(
    cutting_table: &CuttingTableCreate,
) -> Result<(), AppError> {
    if cutting_table.name.trim().is_empty() {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo obrigatório: nome"));
    }
    if cutting_table.width <= 0 {
        return Err(AppError::new(
            DEFAULT_ERROR_CODE,
            "Campo deve ser maior que zero: largura",
        ));
    }
    if cutting_table.length <= 0 {
        return Err(AppError::new(
            DEFAULT_ERROR_CODE,
            "Campo deve ser maior que zero: comprimento",
        ));
    }
    Ok(())
}

pub async fn update(
    cutting_table: CuttingTable,
    poll: &Pool<Sqlite>,
) -> Result<CuttingTable, AppError> {
    let trimmed_cutting_table = trim_create_cutting_table_fields(cutting_table).clone();

    verify_create_cutting_table_fields(&trimmed_cutting_table)?;

    let result = cutting_table::update(trimmed_cutting_table, poll).await;

    match result {
        Ok(table) => Ok(table),
        Err(_error) => Err(AppError::new(DEFAULT_ERROR_CODE, "Falha ao salvar Mesa")),
    }
}

fn trim_create_cutting_table_fields(cutting_table: CuttingTable) -> CuttingTable {
    let mut new_cutting_table = cutting_table;
    new_cutting_table.name = new_cutting_table.name.trim().to_owned();

    new_cutting_table
}

fn verify_create_cutting_table_fields(cutting_table: &CuttingTable) -> Result<(), AppError> {
    if cutting_table.name.trim().is_empty() {
        return Err(AppError::new(DEFAULT_ERROR_CODE, "Campo obrigatório: nome"));
    }
    if cutting_table.width <= 0 {
        return Err(AppError::new(
            DEFAULT_ERROR_CODE,
            "Campo deve ser maior que zero: largura",
        ));
    }
    if cutting_table.length <= 0 {
        return Err(AppError::new(
            DEFAULT_ERROR_CODE,
            "Campo deve ser maior que zero: comprimento",
        ));
    }
    Ok(())
}
