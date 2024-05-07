use crate::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, sqlite::SqliteQueryResult, Acquire, Pool, Sqlite};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Service {
    id: Option<u32>,
    description: String,
    price: u32,
}

impl Service {
    #[allow(clippy::too_many_arguments)]
    pub fn new(description: String, price: u32) -> Self {
        Self {
            id: None,
            description,
            price,
        }
    }

    pub async fn create_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS service (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                price INTEGER NOT NULL
            )
        ";
        sqlx::query(query).execute(pool).await.or_else(Error::sqlx)
    }

    pub async fn create(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        // let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let mut tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await.or_else(Error::sqlx)?;
        if let Err(err) = sqlx::query("INSERT INTO service (description, price) VALUES ($1, $2)")
            .bind(&self.description)
            .bind(self.price)
            .execute(&mut tx)
            .await
        {
            tx.rollback().await.or_else(Error::sqlx)?;
            return Error::sqlx(err);
        };
        let last_inserted_id: u32 = match sqlx::query_scalar(
            "
            SELECT last_insert_rowid()
        ",
        )
        .fetch_one(&mut tx)
        .await
        {
            Ok(id) => id,
            Err(err) => {
                tx.rollback().await.or_else(Error::sqlx)?;
                return Error::sqlx(err);
            }
        };
        tx.commit().await.or_else(Error::sqlx)?;
        self.id = Some(last_inserted_id);

        Ok(())
    }

    pub async fn read(id: u32, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let service: Service =
            sqlx::query_as("SELECT id, description, price FROM service WHERE id = ?")
                .bind(id)
                .fetch_one(&mut conn)
                .await
                .or_else(Error::sqlx)?;
        Ok(service)
    }

    pub async fn read_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let services: Vec<Service> = sqlx::query_as("SELECT * FROM service")
            .fetch_all(&mut conn)
            .await
            .or_else(Error::sqlx)?;
        Ok(services)
    }
}
