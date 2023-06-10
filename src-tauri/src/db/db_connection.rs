use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

const CREATE_FABRIC_SCHEMA_SQL: &str = 
"CREATE TABLE IF NOT EXISTS fabric (
    id INTEGER PRIMARY KEY NOT NULL, 
    name VARCHAR(250) NOT NULL,
    manufacturer VARCHAR(250),
    width INTEGER NOT NULL,
    code VARCHAR(250)
);";

const DEV_POPULATE_FABRIC_SQL: &str = 
"INSERT INTO fabric (name, manufacturer, width, code) Values('Tecido Normal', 'Fabricante 1', 4000, '23dfasdv4crgfd');
INSERT INTO fabric (name, manufacturer, width, code) Values('Tecido Largo', 'Fabricante 1', 5000, 'sdasdasdasd876678');";

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

    execute_query(CREATE_FABRIC_SCHEMA_SQL, &db_pool).await;

    if cfg!(dev) {
        execute_query(DEV_POPULATE_FABRIC_SQL, &db_pool).await;
    }

    db_pool
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

async fn execute_query(query: &str, pool: &Pool<Sqlite>) {
    let result = sqlx::query(query
    )
    .execute(pool)
    .await
    .unwrap_or_else(|_| panic!("Failed to run query: {}", query));
    println!("Create schema: {:?}", result);
}