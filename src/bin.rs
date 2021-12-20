use clap::Parser;
use std::error::Error;
use kollider_api::kollider::api::*;
use kollider_api::kollider::client::*;
use chrono::prelude::*;
use chrono::Duration;
use do_notation::m;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    testnet: bool,
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: Option<String>,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: Option<String>,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn require_auth(args: &Args) -> Result<KolliderAuth, base64::DecodeError> {
    let mauth = m! {
        api_key <- args.api_key.as_ref();
        api_secret <- args.api_secret.as_ref();
        password <- args.password.as_ref();
        Some(KolliderAuth::new(api_key, api_secret, password))
    };
    mauth.expect("We require auth information for that endpoint. Provide api_key, api_secret and password, please.")
}

#[derive(Parser, Debug)]
enum SubCommand {
    Products,
    Orderbook(OrderbookCmd),
    Ticker(TickerCmd),
    History(HistoryCmd),
    Account(AccountCmd),
    #[clap(subcommand)]
    Deposit(DepositSub),
    #[clap(subcommand)]
    Withdrawal(WithdrawalSub)
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

#[derive(Parser, Debug)]
struct AccountCmd {

}

#[derive(Parser, Debug)]
enum DepositSub {
    /// Deposit Bitcoins onchain. Non operational endpoint for now
    Btc(DepositBtc),
    /// Deposit Bitcoins using Lightning Network. The method returns an invoice that you should pay with LN wallet as usual.
    Ln(DepositLn),
}

#[derive(Parser, Debug)]
struct DepositBtc {

}

#[derive(Parser, Debug)]
struct DepositLn {
    #[clap(long, help = "Amount of deposit in sats")]
    amount: u64,
}

#[derive(Parser, Debug)]
enum WithdrawalSub {
    /// Withdrawal of Bitcoins onchain
    Btc(WithdrawalBtc),
    /// Withdrawal of Bitcoins using Lightning Network.
    Ln(WithdrawalLn),
}

#[derive(Parser, Debug)]
struct WithdrawalBtc {
    /// BTC receiving address
    address: String,
    #[clap(long, help = "Amount of withdrawal in sats")]
    amount: u64,
}

#[derive(Parser, Debug)]
struct WithdrawalLn {
    /// Payment request
    invoice: String,
    #[clap(long, help = "Amount of withdrawal in sats")]
    amount: u64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::init();

    let mut client = if args.testnet {
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
        SubCommand::Account(AccountCmd{}) => {
            let auth = require_auth(&args)?;
            client.auth = Some(auth);
            let resp = client.user_account().await?;
            println!("Response /user/account: {:?}", resp);
        }
        SubCommand::Deposit(ref deposit_sub) => match deposit_sub {
            DepositSub::Btc(DepositBtc{}) => {
                let auth = require_auth(&args)?;
                client.auth = Some(auth);
                let resp = client.wallet_deposit(&DepositBody::Bitcoin).await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
            DepositSub::Ln(DepositLn{amount}) => {
                let auth = require_auth(&args)?;
                client.auth = Some(auth);
                let resp = client.wallet_deposit(&DepositBody::Lighting(*amount)).await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
        }
        SubCommand::Withdrawal(ref withdrawal_sub) => match withdrawal_sub {
            WithdrawalSub::Btc(WithdrawalBtc{address, amount}) => {
                let auth = require_auth(&args)?;
                client.auth = Some(auth);
                let resp = client.wallet_withdrawal(&WithdrawalBody::Bitcoin {
                    _type: BtcTag::BTC,
                    receive_address: address.clone(),
                    amount: *amount
                }).await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
            WithdrawalSub::Ln(WithdrawalLn{invoice, amount}) => {
                let auth = require_auth(&args)?;
                client.auth = Some(auth);
                let resp = client.wallet_withdrawal(&WithdrawalBody::Lighting {
                    _type: LnTag::Ln,
                    payment_request: invoice.clone(),
                    amount: *amount
                }).await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
        }
    }

    Ok(())
}