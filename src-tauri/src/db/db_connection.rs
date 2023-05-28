use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub struct DbConnection {
    pub db: Pool<Sqlite>,
}

impl DbConnection {
    pub async fn new() -> Result<Self, ()> {
        Ok(DbConnection {
            db: init_db().await,
        })
    }
}

async fn init_db() -> Pool<Sqlite> {
    if cfg!(dev) {
        drop_db().await;
    }
    create_db().await;

    let db_pool = get_poll().await;

    db_pool.to_owned()
}

async fn drop_db() {
    if Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Droping database {}", DB_URL);
        match Sqlite::drop_database(DB_URL).await {
            Ok(_) => println!("Drop db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database does not exists");
    }
}

async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

async fn get_poll() -> Pool<Sqlite> {
    SqlitePool::connect(DB_URL).await.expect("Failed to connect to db")
}