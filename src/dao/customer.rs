use crate::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{
    pool::{Pool, PoolConnection},
    sqlite::Sqlite,
    sqlite::SqliteQueryResult,
    Acquire,
};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Customer {
    id: Option<u32>,
    name: String,
    gender: String,
    birth_date: String,
    phone: String,
    email: String,
    rg: String,
    address: String,
    cep: String,
}

impl Customer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        gender: String,
        birth_date: String,
        phone: String,
        email: String,
        rg: String,
        address: String,
        cep: String,
    ) -> Self {
        Self {
            id: None,
            name,
            gender,
            birth_date,
            phone,
            email,
            rg,
            address,
            cep,
        }
    }

    // pub fn get_id(&self) -> u32 {
    //     self.id.unwrap()
    // }

    pub async fn create_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS customer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                gender TEXT NOT NULL,
                birth_date TEXT NOT NULL,
                phone TEXT NOT NULL,
                email TEXT NOT NULL,
                rg TEXT NOT NULL,
                address TEXT NOT NULL,
                cep TEXT NOT NULL
            )
        ";
        sqlx::query(query).execute(pool).await.or_else(Error::sqlx)
    }

    pub async fn create(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let mut tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await.or_else(Error::sqlx)?;

        if let Err(err) = sqlx::query("INSERT INTO customer (name, gender, birth_date, phone, email, rg, address, cep) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&self.name)
            .bind(&self.gender)
            .bind(&self.birth_date)
            .bind(&self.phone)
            .bind(&self.email)
            .bind(&self.rg)
            .bind(&self.address)
            .bind(&self.cep)
            .execute(&mut tx)
            .await {
                tx.rollback().await.or_else(Error::sqlx)?;
                return Error::sqlx(err);
            }

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

    pub async fn read(id: &u32, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let customer: Customer = sqlx::query_as("SELECT id, name, gender, birth_date, phone, email, rg, address, cep FROM customer WHERE  id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .or_else(Error::sqlx)?;
        Ok(customer)
    }

    pub async fn read_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let customers: Vec<Customer> = sqlx::query_as("SELECT * FROM customer")
            .fetch_all(&mut conn)
            .await
            .or_else(Error::sqlx)?;
        Ok(customers)
    }
}
