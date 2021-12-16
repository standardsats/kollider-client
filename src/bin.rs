use clap::Parser;
use std::error::Error;
use kollider_api::kollider::api::*;
use kollider_api::kollider::client::*;
use chrono::prelude::*;
use chrono::Duration;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    testnet: bool,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Products,
    Orderbook(OrderbookCmd),
    Ticker(TickerCmd),
    History(HistoryCmd),
}

#[derive(Parser, Debug)]
struct OrderbookCmd {
    #[clap(short, long, default_value="2")]
    level: u64,
    #[clap(short, long, default_value="BTCUSD.PERP")]
    symbol: String,
}

#[derive(Parser, Debug)]
struct TickerCmd {
    #[clap(short, long, default_value="BTCUSD.PERP")]
    symbol: String,
}

#[derive(Parser, Debug)]
struct HistoryCmd {
    #[clap(short, long, default_value="100")]
    limit: usize,
    #[clap(short, long, default_value="BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(short, long, default_value="5m")]
    interval: IntervalSize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = if args.testnet {
        KolliderClient::testnet()
    } else {
        KolliderClient::mainnet()
    };

    match args.subcmd {
        SubCommand::Products => {
            let resp = client.market_products().await?;
            println!("Response /market/products: {:?}", resp);
        }
        SubCommand::Orderbook(OrderbookCmd{level, symbol}) => {
            let book_level = OrderBookLevel::from_int(level).expect("Order book level is either 2 or 3");
            let resp = client.market_orderbook(book_level, &symbol).await?;
            println!("Response /market/orderbook: {:?}", resp);
        }
        SubCommand::Ticker(TickerCmd{symbol}) => {
            let resp = client.market_ticker(&symbol).await?;
            println!("Response /market/ticker: {:?}", resp);
        }
        SubCommand::History(HistoryCmd{limit, symbol, start, end, interval}) => {
            let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
            let end_time = end.unwrap_or_else(|| Local::now());
            let resp = client.market_historic_index_prices(limit, &symbol, start_time, end_time, interval).await?;
            println!("Response /market/historic_index_prices: {:?}", resp);
        }
    }

    Ok(())
}