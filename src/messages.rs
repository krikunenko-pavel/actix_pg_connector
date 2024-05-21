use actix::Message;
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::tokio_postgres::types::ToSql;

pub struct SelectOneMessage(
    pub String,
    pub Vec<Box<(dyn ToSql + Sync )>>,
);

impl Message for SelectOneMessage {
    type Result = Result<Row, deadpool_postgres::tokio_postgres::Error>;
}