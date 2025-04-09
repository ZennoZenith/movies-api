use axum::{Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    configuration::{DatabaseSettings, Settings},
    utils::{using_connection_extractor, using_connection_pool_extractor},
};

pub struct Application;

impl Application {
    pub async fn run_until_stopped(configuration: &Settings) -> Result<(), std::io::Error> {
        let pool = get_connection_pool(&configuration.database).await;
        // sqlx::migrate!().run(&pool).await?;

        let address = (
            configuration.application.host.as_str(),
            configuration.application.port,
        );
        let listener = tokio::net::TcpListener::bind(address).await?;

        tracing::debug!("listening on {}", listener.local_addr().unwrap());

        let app = Router::new()
            .route(
                "/",
                get(using_connection_pool_extractor).post(using_connection_extractor),
            )
            .with_state(pool);

        axum::serve(listener, app).await?;
        Ok(())
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(configuration.max_connections)
        .connect_with(configuration.with_db())
        .await
        .unwrap()
}
