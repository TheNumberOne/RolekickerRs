use tokio_postgres::NoTls;

use std::env;
use log::{info, error};

mod migrations;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("database url");
    let (mut client, connection) =
        tokio_postgres::connect(&db_url, NoTls).await.expect("Can connect to db");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    let migration_report = migrations::runner().run_async(&mut client).await.expect("Can run migrations");

    for migration in migration_report.applied_migrations() {
        info!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    info!("DB migrations finished!");
}
