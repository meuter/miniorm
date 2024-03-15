use crate::{
    miniorm::{Db, Store, Table},
    model::Transaction,
};
use async_trait::async_trait;

pub struct TransactionStore;

#[async_trait]
impl Store<Transaction> for TransactionStore {
    const TABLE: Table = Table(
        "transaction",
        &[
            ("date", "DATE NOT NULL"),
            ("operation", "JSONB NOT NULL"),
            ("instrument", "JSONB NOT NULL"),
            ("quantity", "DECIMAL NOT NULL"),
            ("unit_price", "DECIMAL NOT NULL"),
            ("taxes", "DECIMAL NOT NULL"),
            ("fees", "DECIMAL NOT NULL"),
            ("currency", "JSONB NOT NULL"),
            ("exchange_rate", "DECIMAL NOT NULL"),
        ],
    );

    async fn update(_db: &Db, _id: i64, _entity: Transaction) -> sqlx::Result<i64> {
        todo!()
    }
}
