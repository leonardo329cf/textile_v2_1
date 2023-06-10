use serde::{Serialize, Deserialize};
use sqlx::{Error, Sqlite, Pool};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Fabric {
    pub id: i32,
    pub name: String,
    pub manufacturer: String,
    pub width: i32,
    pub code: String,
}

pub async fn get(id: i32, poll: &Pool<Sqlite>) -> Result<Fabric, Error> {
    sqlx::query_as::<Sqlite, Fabric>("SELECT * FROM fabric WHERE id = ?")
        .bind(id)
        .fetch_one(poll)
        .await
}

pub async fn get_all(poll: &Pool<Sqlite>) -> Result<Vec<Fabric>, Error> {
    sqlx::query_as::<Sqlite, Fabric>("SELECT * FROM fabric")
        .fetch_all(poll)
        .await
}

pub async fn delete(id: i32, poll: &Pool<Sqlite>) -> Result<Fabric, Error> {
    sqlx::query_as::<Sqlite, Fabric>("DELETE FROM fabric WHERE id = ? RETURNING *;")
        .bind(id)
        .fetch_one(poll)
        .await
}

pub async fn create(fabric: FabricCreate, poll: &Pool<Sqlite>) -> Result<Fabric, Error> {
    sqlx::query_as::<Sqlite, Fabric>("INSERT INTO fabric (name, manufacturer, width, code) Values(?, ?, ?, ?) RETURNING *;")
        .bind(fabric.name)
        .bind(fabric.manufacturer)
        .bind(fabric.width)
        .bind(fabric.code)
        .fetch_one(poll)
        .await
}

pub async fn update(fabric: Fabric, poll: &Pool<Sqlite>) -> Result<Fabric, Error> {
    sqlx::query_as::<Sqlite, Fabric>("UPDATE fabric SET name = ?, manufacturer = ?, width = ?, code = ? WHERE id = ? RETURNING *;")
        .bind(fabric.name)
        .bind(fabric.manufacturer)
        .bind(fabric.width)
        .bind(fabric.code)
        .bind(fabric.id)
        .fetch_one(poll)
        .await
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricCreate {
    pub name: String,
    pub manufacturer: String,
    pub width: i32,
    pub code: String,
}