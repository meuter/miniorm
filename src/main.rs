mod miniorm;
mod model;
mod store;

use dotenv::dotenv;
use iso_currency::Currency;
use model::Transaction;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::{types::chrono::NaiveDate, PgPool};

use crate::{
    miniorm::Table,
    model::{Instrument, Operation, Stock, Ticker},
    store::TransactionTable,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env");
    let pool = PgPool::connect(&url).await?;

    TransactionTable::recreate(&pool).await?;

    let aapl = Stock {
        ticker: Ticker("AAPL".into()),
        currency: Currency::USD,
    };

    let tx = Transaction {
        date: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        operation: Operation::Buy,
        instrument: Instrument::Stock(aapl),
        quantity: dec!(10),
        unit_price: dec!(170.0),
        taxes: dec!(10.2),
        fees: dec!(5.5),
        currency: Currency::USD,
        exchange_rate: dec!(0.9),
    };

    let id = TransactionTable::create(&pool, &tx).await?;
    let fetched = TransactionTable::read(&pool, id).await?;
    assert_eq!(tx, fetched);

    let all = TransactionTable::list(&pool).await?;
    assert_eq!(vec![tx], all);

    let deleted = TransactionTable::delete(&pool, id).await?;
    assert_eq!(deleted, 1);

    assert!(matches!(
        TransactionTable::read(&pool, id).await,
        Err(sqlx::Error::RowNotFound)
    ));

    Ok(())
}
