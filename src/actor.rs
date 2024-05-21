use actix;
use actix::{Actor, Context, Handler, ResponseFuture};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use deadpool_postgres::tokio_postgres::{NoTls, Row};
use deadpool_postgres::tokio_postgres::types::ToSql;
use url::Url;

use crate::messages::SelectOneMessage;

pub struct PgConnector(Pool);

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

        Self(
            cfg.create_pool(Some(Runtime::Tokio1), NoTls).expect("Create pool error")
        )
    }
}

impl Actor for PgConnector {
    type Context = Context<Self>;
}

impl Handler<Box<SelectOneMessage>> for PgConnector {
    type Result = ResponseFuture<Result<Row, deadpool_postgres::tokio_postgres::Error>>;

    fn handle(&mut self, msg: Box<SelectOneMessage>, _: &mut Self::Context) -> Self::Result {
        let query = msg.0.clone();
        let params: Vec<Box<(dyn ToSql + Sync)>> = msg.1.into_iter().collect();
        let pool = self.0.clone();

        Box::pin(async move {
            let inner_params: Vec<&(dyn ToSql + Sync)> = params.iter()
                .map(
                    |param| param.as_ref()
                )
                .collect();
            let conn = pool.get().await.unwrap();
            let stmt = conn.prepare(&query).await.unwrap();
            let row = conn.query_one(
                &stmt,
                &inner_params[..],
            ).await.unwrap();

            Ok(row)
        })
    }
}