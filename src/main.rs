use chrono::{DateTime, NaiveDate, Utc};
use dotenv::dotenv;
use std::{env, option::Option};
use tokio_postgres::NoTls;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connect_string = format!(
        "host={host} port={port} user={user} password={pass} dbname={name}",
        host = env::var("DATABASE_HOST").unwrap(),
        port = env::var("DATABASE_PORT").unwrap(),
        user = env::var("DATABASE_USERNAME").unwrap(),
        pass = env::var("DATABASE_PASSWORD").unwrap(),
        name = env::var("DATABASE_NAME").unwrap(),
    );

    let (client, connection) = tokio_postgres::connect(&connect_string, NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("{:?}", e);
        }
    });

    let rows = client.query("SELECT * FROM users", &[]).await.unwrap();
    for r in rows {
        let id: i32 = r.get(0);
        let first_name: String = r.get(1);
        let last_name: String = r.get(2);
        let email: String = r.get(3);
        let date_of_birth: Option<NaiveDate> = r.get(4);
        let created_at: DateTime<Utc> = r.get(5);
        println!(
            "{:?} {:?} {:?} {:?} {:?} {:?}",
            id, first_name, last_name, email, date_of_birth, created_at
        ); // "Jim" "Pam"
    }
}
