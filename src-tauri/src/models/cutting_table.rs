use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Sqlite};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CuttingTable {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub length: i32,
}

pub async fn get(id: i32, poll: &Pool<Sqlite>) -> Result<CuttingTable, Error> {
    sqlx::query_as::<Sqlite, CuttingTable>("SELECT * FROM cutting_table WHERE id = ?")
        .bind(id)
        .fetch_one(poll)
        .await
}

pub async fn get_all(poll: &Pool<Sqlite>) -> Result<Vec<CuttingTable>, Error> {
    sqlx::query_as::<Sqlite, CuttingTable>("SELECT * FROM cutting_table")
        .fetch_all(poll)
        .await
}

pub async fn delete(id: i32, poll: &Pool<Sqlite>) -> Result<CuttingTable, Error> {
    sqlx::query_as::<Sqlite, CuttingTable>("DELETE FROM cutting_table WHERE id = ? RETURNING *;")
        .bind(id)
        .fetch_one(poll)
        .await
}

pub async fn create(
    cutting_table: CuttingTableCreate,
    poll: &Pool<Sqlite>,
) -> Result<CuttingTable, Error> {
    sqlx::query_as::<Sqlite, CuttingTable>(
        "INSERT INTO cutting_table (name, width, length) Values(?, ?, ?) RETURNING *;",
    )
    .bind(cutting_table.name)
    .bind(cutting_table.width)
    .bind(cutting_table.length)
    .fetch_one(poll)
    .await
}

pub async fn update(
    cutting_table: CuttingTable,
    poll: &Pool<Sqlite>,
) -> Result<CuttingTable, Error> {
    sqlx::query_as::<Sqlite, CuttingTable>(
        "UPDATE cutting_table SET name = ?, width = ?, length = ? WHERE id = ? RETURNING *;",
    )
    .bind(cutting_table.name)
    .bind(cutting_table.width)
    .bind(cutting_table.length)
    .bind(cutting_table.id)
    .fetch_one(poll)
    .await
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CuttingTableCreate {
    pub name: String,
    pub width: i32,
    pub length: i32,
}
