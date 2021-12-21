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
    /// Print available tickers
    Products,
    /// Get info from public orderbook
    Orderbook(OrderbookCmd),
    /// Get info about given ticker symbol
    Ticker(TickerCmd),
    /// Get historical data about prices (not operational)
    History(HistoryCmd),
    /// Get information about an account. Requires authentification.
    Account(AccountCmd),
    /// Deposit money to an account. Requires authentification.
    #[clap(subcommand)]
    Deposit(DepositSub),
    /// Withdraw money from an account. Requires authentification.
    #[clap(subcommand)]
    Withdrawal(WithdrawalSub),
    /// Manipulate orderbook for given account. Requires authentification.
    #[clap(subcommand)]
    Order(OrderSub)
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
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
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
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
}

#[derive(Parser, Debug)]
struct DepositLn {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
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
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    /// BTC receiving address
    address: String,
    #[clap(long, help = "Amount of withdrawal in sats")]
    amount: u64,
}

#[derive(Parser, Debug)]
struct WithdrawalLn {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    /// Payment request
    invoice: String,
    #[clap(long, help = "Amount of withdrawal in sats")]
    amount: u64,
}

#[derive(Parser, Debug)]
enum OrderSub {
    /// Place new order in book
    Create(OrderCreateCmd),
    /// Preflight information for creating a new order
    Prediction(OrderCreateCmd),
    /// List historic info about orders of the account
    List(OrderListCmd),
    /// List opened orders of the account
    Opened(OrderOpenedCmd),
    /// List fill info about user orders
    Fills(OrderFillsCmd),
}

#[derive(Parser, Debug)]
struct OrderCreateCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(long, default_value="BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    quantity: u64,
    #[clap(long)]
    price: u64,
    #[clap(long, default_value="1")]
    leverage: u64,
    #[clap(long)]
    side: OrderSide,
    #[clap(long, default_value="Isolated")]
    margin_type: MarginType,
    #[clap(long, default_value="Limit")]
    order_type: OrderType,
    #[clap(long, default_value="Delayed")]
    settlement_type: SettlementType,
}

#[derive(Parser, Debug)]
struct OrderListCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(long, default_value="BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(long, default_value="100")]
    limit: usize,
}

#[derive(Parser, Debug)]
struct OrderOpenedCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
}

#[derive(Parser, Debug)]
struct OrderFillsCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(long, default_value="BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(long, default_value="100")]
    limit: usize,
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
        SubCommand::Account(AccountCmd{api_key, api_secret, password}) => {
            let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
            client.auth = Some(auth);
            let resp = client.user_account().await?;
            println!("Response /user/account: {:?}", resp);
        }
        SubCommand::Deposit(ref deposit_sub) => match deposit_sub {
            DepositSub::Btc(DepositBtc{api_key, api_secret, password}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.wallet_deposit(&DepositBody::Bitcoin).await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
            DepositSub::Ln(DepositLn{api_key, api_secret, password, amount}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.wallet_deposit(&DepositBody::Lighting(*amount)).await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
        }
        SubCommand::Withdrawal(ref withdrawal_sub) => match withdrawal_sub {
            WithdrawalSub::Btc(WithdrawalBtc{api_key, api_secret, password, address, amount}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.wallet_withdrawal(&WithdrawalBody::Bitcoin {
                    _type: BtcTag::BTC,
                    receive_address: address.clone(),
                    amount: *amount
                }).await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
            WithdrawalSub::Ln(WithdrawalLn{api_key, api_secret, password, invoice, amount}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.wallet_withdrawal(&WithdrawalBody::Lighting {
                    _type: LnTag::Ln,
                    payment_request: invoice.clone(),
                    amount: *amount
                }).await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
        }
        SubCommand::Order(order_sub) => match order_sub {
            OrderSub::Create(OrderCreateCmd {api_key, api_secret, password, symbol, quantity, price, leverage, side, margin_type, order_type, settlement_type}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.create_order(&OrderBody {
                    symbol, quantity, price, leverage, side, margin_type, order_type, settlement_type
                }).await?;
                println!("Response /orders: {:?}", resp);
            }
            OrderSub::Prediction(OrderCreateCmd {api_key, api_secret, password, symbol, quantity, price, leverage, side, margin_type, order_type, settlement_type}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.order_prediction(&OrderBody {
                    symbol, quantity, price, leverage, side, margin_type, order_type, settlement_type
                }).await?;
                println!("Response /orders/prediction: {:?}", resp);
            }
            OrderSub::List(OrderListCmd {api_key, api_secret, password, symbol, start, end, limit}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
                let end_time = end.unwrap_or_else(|| Local::now());
                let resp = client.orders(&symbol, start_time, end_time, limit).await?;
                println!("Response /orders: {:?}", resp);
            }
            OrderSub::Opened(OrderOpenedCmd {api_key, api_secret, password}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.open_orders().await?;
                println!("Response /orders/opened: {:?}", resp);
            }
            OrderSub::Fills(OrderFillsCmd {api_key, api_secret, password, symbol, start, end, limit}) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
                let end_time = end.unwrap_or_else(|| Local::now());
                let resp = client.fills(&symbol, start_time, end_time, limit).await?;
                println!("Response /user/fills: {:?}", resp);
            }
        }
    }

    Ok(())
}