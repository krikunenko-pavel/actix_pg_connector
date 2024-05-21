use actix::Message;
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::tokio_postgres::types::ToSql;

pub type ReturnOneResult = Result<Row, deadpool_postgres::tokio_postgres::Error>;


pub struct SelectOneMessage {
    pub query: String,
    pub values: Vec<Box<(dyn ToSql + Sync )>>,
}

impl Message for SelectOneMessage {
    type Result = ReturnOneResult;
}