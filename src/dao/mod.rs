use sqlx::{Pool, Sqlite, SqlitePool};
use std::fs::File;

mod customer;
mod schedule;
mod service;
mod user;
pub use customer::Customer;
pub use schedule::Schedule;
pub use service::Service;
pub use user::User;

use crate::error::Error;

const DBPATH: &str = "./data/users.db";
const DBURL: &str = "./data/users.db";

pub struct Dao {
    pool: Pool<Sqlite>,
}

impl Dao {
    pub async fn new() -> Result<Self, Error> {
        // let db_location = DBPATH;
        // let db_location = format!("sqlite:{DBPATH}");
        match File::create(DBPATH) {
            Ok(_) => {}
            Err(err) => {
                panic!("Error when attempting to read or create database file. Message: {err}");
            }
        };

        // let pool: Pool<Sqlite> = SqlitePool::connect(&db_location).await.unwrap();

        let pool: Pool<Sqlite> = SqlitePool::connect(DBURL).await.unwrap();
        // Create tables
        Customer::create_table(&pool).await?;
        Service::create_table(&pool).await?;
        let mut service = Service::new("Corte simples".to_string(), 2500);
        service.create(&pool).await.unwrap();
        User::create_table(&pool).await?;
        Schedule::create_table(&pool).await?;

        Ok(Self { pool })
    }

    pub fn get(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
