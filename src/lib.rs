pub extern crate actix;
pub extern crate deadpool_postgres;

use actix::{Actor, Context};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use deadpool_postgres::tokio_postgres::NoTls;
use url::Url;

pub struct PgConnector {
    pool: Pool,
}

impl PgConnector {
    pub fn new(db_url: &str) -> Self {
        let url = Url::parse(db_url).expect("Not valid url");

        let mut cfg = Config::new();
        cfg.dbname = Some(url.path().parse().expect("No pg dbname"));
        cfg.user = Some(url.username().parse().expect("No pg username"));
        cfg.password = Some(url.password().expect("No pg password").parse().unwrap());
        cfg.host = Some(url.host().expect("No pg host").to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        Self {
            pool: cfg.create_pool(Some(Runtime::Tokio1), NoTls).expect("Create pool error")
        }
    }
}

impl Actor for PgConnector {
    type Context = Context<Self>;
}

#[cfg(test)]
mod tests {}
