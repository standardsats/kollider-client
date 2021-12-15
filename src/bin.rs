use clap::Parser;
use std::error::Error;
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
    }

    Ok(())
}