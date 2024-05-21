use actix::Message;
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::tokio_postgres::types::ToSql;

pub struct SelectOneMessage<'a>(
    pub String,
    pub Vec<Box<(dyn ToSql + Sync + 'a)>>,
);

impl Message for SelectOneMessage<'_> {
    type Result = Result<Row, deadpool_postgres::tokio_postgres::Error>;
}