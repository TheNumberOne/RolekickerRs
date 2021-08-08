use std::env;
use log::{info, error};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

mod migrations;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("database url");
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("ssl");
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let (mut client, connection) =
        tokio_postgres::connect(&db_url, connector).await.expect("Can connect to db");

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
