use clap::Parser;
use std::error::Error;
use kollider_api::kollider::api::*;
use kollider_api::kollider::client::*;

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
}

#[derive(Parser, Debug)]
struct OrderbookCmd {
    #[clap(short, long, default_value="2")]
    level: u64,
    #[clap(short, long, default_value="BTCUSD.PERP")]
    symbol: String,
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
            let book_level = OrderBookLevel::from_int(level).unwrap();
            let resp = client.market_orderbook(book_level, &symbol).await?;
            println!("Response /market/orderbook: {:?}", resp);
        }
    }

    Ok(())
}