use crate::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{pool::Pool, sqlite::Sqlite, Transaction};

use super::customer::Customer;
use super::service::Service;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
struct ScheduleTable {
    id: Option<u32>,
    customer_id: u32,
    service_id: u32,
    price: u32,
    date: String,
    hour: u32,
    details: String,
}

impl ScheduleTable {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        customer_id: u32,
        service_id: u32,
        price: u32,
        date: String,
        hour: u32,
        details: String,
    ) -> Self {
        Self {
            id: None,
            customer_id,
            service_id,
            price,
            date,
            hour,
            details,
        }
    }

    // pub fn get_id(&self) -> u32 {
    //     self.id.unwrap()
    // }

    pub async fn create_table(pool: &Pool<Sqlite>) -> Result<(), Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS schedule (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id INTEGER NOT NULL,
                service_id INTEGER NOT NULL,
                price INTEGER NOT NULL,
                date TEXT NOT NULL,
                hour INTEGER NOT NULL,
                details TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customer(id),
                FOREIGN KEY (service_id) REFERENCES service(id)
            )
        ";
        sqlx::query(query)
            .execute(pool)
            .await
            .or_else(Error::sqlx)?;
        Ok(())
    }

    // pub async fn create<'a>(
    //     &'a self,
    //     mut tx: sqlx::Transaction<'a, Sqlite>,
    // ) -> Result<(sqlx::Transaction<Sqlite>, u32), Error> {
    //     // let mut conn: PoolConnection<Sqlite> = pool.acquire().await.or_else(Error::sqlx)?;
    //     // let mut tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await.or_else(Error::sqlx)?;

    //     if let Err(err) = sqlx::query("INSERT INTO schedule (customer_id, service_id, price, date, hour, details) VALUES ($1, $2, $3, $4, $5, $6)")
    //         .bind(self.customer_id)
    //         .bind(self.service_id)
    //         .bind(self.price)
    //         .bind(&self.date)
    //         .bind(self.hour)
    //         .bind(&self.details)
    //         .execute(&mut tx)
    //         .await {
    //             tx.rollback().await.or_else(Error::sqlx)?;
    //             return Error::sqlx(err);
    //         }

    //     let last_inserted_id: u32 = match sqlx::query_scalar(
    //         "
    //             SELECT last_insert_rowid()
    //         ",
    //     )
    //     .fetch_one(&mut tx)
    //     .await
    //     {
    //         Ok(id) => id,
    //         Err(err) => {
    //             tx.rollback().await.or_else(Error::sqlx)?;
    //             return Error::sqlx(err);
    //         }
    //     };
    //     // tx.commit().await.or_else(Error::sqlx)?;
    //     // self.id = Some(last_inserted_id);

    //     Ok((tx, last_inserted_id))
    // }

    pub async fn retrieve<'a>(
        self,
        mut tx: sqlx::Transaction<'_, Sqlite>,
    ) -> Result<(Schedule, sqlx::Transaction<'_, Sqlite>), Error> {
        let customer_id = self.customer_id;
        let service_id = self.service_id;
        let customer: Customer = sqlx::query_as("SELECT * FROM customer WHERE id = $1")
            .bind(customer_id)
            .fetch_one(&mut tx)
            .await
            .or_else(Error::sqlx)?;

        let service: Service = sqlx::query_as("SELECT * FROM service WHERE id = $1")
            .bind(service_id)
            .fetch_one(&mut tx)
            .await
            .or_else(Error::sqlx)?;

        let id = match self.id {
            Some(id) => id,
            None => {
                // let (new_tx, id) = self.clone().create(tx).await?;
                // tx = other.create(tx).await?;
                // (new_tx, id)
                if let Err(err) = sqlx::query("INSERT INTO schedule (customer_id, service_id, price, date, hour, details) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(self.customer_id)
                    .bind(self.service_id)
                    .bind(self.price)
                    .bind(&self.date)
                    .bind(self.hour)
                    .bind(&self.details)
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
                last_inserted_id
            }
        };

        let schedule = Schedule {
            id,
            customer,
            service,
            price: self.price,
            date: self.date,
            hour: self.hour,
            details: self.details,
        };

        // let schedule: Schedule = Schedule::new(
        //     self.id.unwrap(),
        //     customer,
        //     service,
        //     self.price,
        //     self.date,
        //     self.hour,
        //     self.details,
        // );

        Ok((schedule, tx))
    }

    pub async fn read(id: &u32, tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<Self, Error> {
        let schedule: Self = sqlx::query_as("SELECT id, customer_id, service_id, price, date, hour, details FROM schedule WHERE  id = ?")
        .bind(id)
        .fetch_one(tx)
        .await
        .or_else(Error::sqlx)?;
        Ok(schedule)
    }

    pub async fn read_all(tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<Vec<Self>, Error> {
        let schedules: Vec<Self> = sqlx::query_as("SELECT * FROM schedule")
            .fetch_all(tx)
            .await
            .or_else(Error::sqlx)?;
        Ok(schedules)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    id: u32,
    customer: Customer,
    service: Service,
    price: u32,
    date: String,
    hour: u32,
    details: String,
}

impl Schedule {
    pub async fn new(
        customer_id: u32,
        service_id: u32,
        price: u32,
        date: String,
        hour: u32,
        details: String,
        tx: Transaction<'_, Sqlite>,
    ) -> Result<(Self, Transaction<Sqlite>), Error> {
        let schedule_table =
            ScheduleTable::new(customer_id, service_id, price, date, hour, details);
        let (schedule, tx) = schedule_table.retrieve(tx).await?;
        Ok((schedule, tx))
    }

    pub async fn create_table(pool: &Pool<Sqlite>) -> Result<(), Error> {
        ScheduleTable::create_table(pool).await
    }

    // pub async fn create(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
    //     let mut schedule_table: ScheduleTable = ScheduleTable::new(
    //         self.customer.get_id(),
    //         self.service.get_id(),
    //         self.price,
    //         self.date.clone(),
    //         self.hour,
    //         self.details.clone(),
    //     );
    //     schedule_table.create(pool).await?;
    //     self.id = schedule_table.get_id();
    //     Ok(())
    // }

    pub async fn read(id: u32, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut tx: Transaction<'_, Sqlite> = pool.begin().await.or_else(Error::sqlx)?;
        let schedule_table: ScheduleTable = ScheduleTable::read(&id, &mut tx).await?;
        let (schedule, _) = schedule_table.retrieve(tx).await?;
        Ok(schedule)
    }

    pub async fn read_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        let mut tx: Transaction<'_, Sqlite> = pool.begin().await.or_else(Error::sqlx)?;
        let schedule_tables: Vec<ScheduleTable> = ScheduleTable::read_all(&mut tx).await?;
        let mut schedules: Vec<Schedule> = Vec::with_capacity(schedule_tables.len());
        for schedule_table in schedule_tables.into_iter() {
            let (schedule, new_tx) = schedule_table.retrieve(tx).await?;
            tx = new_tx;
            schedules.push(schedule);
        }
        Ok(schedules)
    }
}
