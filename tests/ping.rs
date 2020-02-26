use binance_async as binance;

use failure::Fallible;

use crate::binance::Binance;

#[tokio::test]
async fn ping() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let binance = Binance::new();

    binance.ping()?.await?;

    Ok(())
}