use std::{path, sync::OnceLock};

use sqlx::Row;
use tracing::event;

use crate::{constant::DATABASE_FILE, models::{HistoryServerFromDatabase, RawHistoryServerFromDatabase, RawServerFromDatabase, Server, ServerFromDatabase}};

#[derive(Debug)]
pub struct Database {
    pool: sqlx::Pool<sqlx::Sqlite>
}

impl Database {
    pub async fn new() -> Self {
        let db_file = path::absolute(DATABASE_FILE.to_path_buf()).unwrap();
        event!(tracing::Level::INFO, "Initializing database at {}", db_file.display());
        // 
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(format!("sqlite:{}", db_file.to_str().unwrap()).as_str()).await.unwrap();
        // favourites
        sqlx::query("CREATE TABLE IF NOT EXISTS servers (address TEXT not null, port INTEGER not null, password TEXT default null)").execute(&pool).await.unwrap();
        // history
        sqlx::query("CREATE TABLE IF NOT EXISTS history_servers (id INTEGER PRIMARY KEY, address TEXT not null, port INTEGER not null, version TEXT not null, timestamp INTEGER not null)").execute(&pool).await.unwrap();
        Self {
            pool,
        }
    }

    pub async fn query_raw_favourites(&self) -> Vec<RawServerFromDatabase> {
        sqlx::query("SELECT address, port, password FROM servers")
            .map(|row: sqlx::sqlite::SqliteRow| RawServerFromDatabase {
                address: row.get("address"),
                port: row.get("port"),
                password: row.get("password"),
            })
            .fetch_all(&self.pool)
            .await.unwrap()
    }

    pub async fn query_favourites(&self) -> Vec<ServerFromDatabase> {
        // try from
        let raw = self.query_raw_favourites().await;
        raw.into_iter().map(ServerFromDatabase::try_from).filter_map(|res| res.ok())
            .collect()
    }

    // check exists
    pub async fn check_favourite_exists(&self, server: &Server) -> bool {
        sqlx::query("SELECT COUNT(1) FROM servers WHERE address = ? AND port = ?")
            .bind(server.ip.to_string())
            .bind(server.port)
            .fetch_one(&self.pool)
            .await
            .unwrap()
            .get::<i64, _>(0) > 0
    }

    pub async fn add_favourite(&self, server: &Server) {
        if self.check_favourite_exists(server).await {
            return;
        }
        sqlx::query("INSERT INTO servers (address, port) VALUES (?, ?)")
            .bind(server.ip.to_string())
            .bind(server.port)
            .execute(&self.pool)
            .await.unwrap();
    }

    pub async fn add_favourite_with_password(&self, server: &Server, password: impl Into<String>) {
        if self.check_favourite_exists(server).await {
            self.set_password(server, password).await;
            return;
        }
        sqlx::query("INSERT INTO servers (address, port, password) VALUES (?, ?, ?)")
            .bind(server.ip.to_string())
            .bind(server.port)
            .bind(password.into())
            .execute(&self.pool)
            .await.unwrap();
    }

    pub async fn remove_favourite(&self, server: &Server) -> bool {
        let res = sqlx::query("DELETE FROM servers WHERE address = ? AND port = ?")
            .bind(server.ip.to_string())
            .bind(server.port)
            .execute(&self.pool)
            .await;
        if let Err(err) = &res {
            event!(tracing::Level::ERROR, "Failed to remove favourite: {}", err);
        }
        res.is_ok()
    }

    // set pass
    pub async fn set_password(&self, server: &Server, password: impl Into<String>) -> bool {
        let res = sqlx::query("UPDATE servers SET password = ? WHERE address = ? AND port = ?")
            .bind(password.into())
            .bind(server.ip.to_string())
            .bind(server.port)
            .execute(&self.pool)
            .await;
        if let Err(err) = &res {
            event!(tracing::Level::ERROR, "Failed to set password: {}", err);
        }
        res.is_ok()
    }

    // insert history logs
    pub async fn insert_history(&self, server: &Server, version: &str) {
        sqlx::query("INSERT INTO history_servers (address, port, version, timestamp) VALUES (?, ?, ?, ?)")
            .bind(server.ip.to_string())
            .bind(server.port)
            .bind(version)
            .bind(chrono::Utc::now().timestamp())
            .execute(&self.pool)
            .await.unwrap();
    }

    pub async fn query_raw_history(&self, page: usize, limit: usize) -> Vec<RawHistoryServerFromDatabase> {
        // sort by timestamp
        sqlx::query("SELECT address, port, version, timestamp FROM history_servers ORDER BY timestamp DESC LIMIT ? OFFSET ?")
            .bind(limit as i32)
            .bind((page * limit) as i32)
            .map(|row: sqlx::sqlite::SqliteRow| RawHistoryServerFromDatabase {
                address: row.get("address"),
                port: row.get("port"),
                version: row.get("version"),
                timestamp: chrono::DateTime::from_timestamp(row.get("timestamp"), 0).unwrap(),
            })
            .fetch_all(&self.pool)
            .await.unwrap()
    }

    pub async fn query_history(&self, page: usize, limit: usize) -> Vec<HistoryServerFromDatabase> {
        let raw = self.query_raw_history(page, limit).await;
        raw.into_iter().map(HistoryServerFromDatabase::try_from).filter_map(|res| res.ok())
            .collect()
    }


    // pub async fn add_favourite(&self, address: &str, port: u16, password: &str) {
        
    // }
}

static DATABASE: OnceLock<Database> = OnceLock::new();

pub fn get_database() -> &'static Database {
    DATABASE.get().unwrap()
}

pub async fn init_database() {
    let database = Database::new().await;
    DATABASE.set(database).unwrap();
}