use crate::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{
    pool::{Pool, PoolConnection},
    sqlite::Sqlite,
    Acquire,
};

#[derive(Deserialize, Serialize)]
/// Doesn't include password
pub struct UserOutput {
    id: u32,
    name: String,
    access: u32,
    gender: String,
    date_of_birth: String,
    phone: String,
    email: String,
    rg: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<u32>,
    name: String,
    password: String,
    access: u32,
    gender: String,
    date_of_birth: String,
    phone: String,
    email: String,
    rg: String,
}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        password: String,
        access: u32,
        gender: String,
        date_of_birth: String,
        phone: String,
        email: String,
        rg: String,
    ) -> Self {
        Self {
            id: None,
            name,
            password,
            access,
            gender,
            date_of_birth,
            phone,
            email,
            rg,
        }
    }

    pub fn output(&self) -> UserOutput {
        UserOutput {
            id: self.id.unwrap(),
            name: self.name.clone(),
            access: self.access,
            gender: self.gender.clone(),
            date_of_birth: self.date_of_birth.clone(),
            phone: self.phone.clone(),
            email: self.email.clone(),
            rg: self.rg.clone(),
        }
    }

    pub fn check_password(&self, password: &str) -> bool {
        self.password.eq(password)
    }

    pub async fn create_table(pool: &Pool<Sqlite>) -> Result<(), Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
                access INTEGER NOT NULL,
                gender TEXT NOT NULL,
                date_of_birth TEXT NOT NULL,
                phone TEXT NOT NULL,
                email TEXT NOT NULL,
                rg TEXT NOT NULL
            )
        ";
        sqlx::query(query)
            .execute(pool)
            .await
            .or_else(Error::sqlx)?;
        let admin_create_query = "INSERT INTO user (name, password, access, gender, date_of_birth, phone, email, rg) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
        sqlx::query(admin_create_query)
            .bind("admin")
            .bind("admin123")
            .bind(1)
            .bind("he")
            .bind("14-02-1994")
            .bind("010 1 718 222 2222")
            .bind("someone@somewhere.com")
            .bind("123456789")
            .execute(pool)
            .await
            .or_else(Error::sqlx)?;
        Ok(())
    }

    pub async fn create(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let mut tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await.or_else(Error::sqlx)?;
        if let Err(err) = sqlx::query("INSERT INTO user (name, password, access, gender, date_of_birth, phone, email, rg) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&self.name)
            .bind(&self.password)
            .bind(self.access)
            .bind(&self.gender)
            .bind(&self.date_of_birth)
            .bind(&self.phone)
            .bind(&self.email)
            .bind(&self.rg)
            .execute(&mut tx)
            .await {
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

    pub async fn read(id: &str, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let user: User = sqlx::query_as("SELECT id, name, password, access, gender, date_of_birth, phone, email, rg FROM user WHERE  id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .or_else(Error::sqlx)?;
        Ok(user)
    }

    pub async fn read_name(name: &str, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let user: User = sqlx::query_as("SELECT id, name, password, access, gender, date_of_birth, phone, email, rg FROM user WHERE  name = ?")
        .bind(name)
        .fetch_one(&mut conn)
        .await
        .or_else(Error::sqlx)?;
        Ok(user)
    }

    pub async fn read_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
        let users: Vec<User> = sqlx::query_as("SELECT * FROM user")
            .fetch_all(&mut conn)
            .await
            .or_else(Error::sqlx)?;
        Ok(users)
    }
}
