use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;
use futures::StreamExt;
use kollider_api::kollider::api::*;
use kollider_api::kollider::client::*;
use kollider_api::kollider::websocket::*;
use kollider_api::kollider::websocket::oneshot::*;
use std::error::Error;
use uuid::Uuid;

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
    /// Get information about balances via sync websocket request.
    Balances(BalancesCmd),
    /// Deposit money to an account. Requires authentification.
    #[clap(subcommand)]
    Deposit(DepositSub),
    /// Withdraw money from an account. Requires authentification.
    #[clap(subcommand)]
    Withdrawal(WithdrawalSub),
    /// Manipulate orderbook for given account. Requires authentification.
    #[clap(subcommand)]
    Order(OrderSub),
    /// Launch a websocket connection and enter iteractive shell.
    #[clap(subcommand)]
    Websocket(WebsocketSub),
}

#[derive(Parser, Debug)]
struct OrderbookCmd {
    #[clap(short, long, default_value = "2")]
    level: u64,
    #[clap(short, long, default_value = "BTCUSD.PERP")]
    symbol: String,
}

#[derive(Parser, Debug)]
struct TickerCmd {
    #[clap(short, long, default_value = "BTCUSD.PERP")]
    symbol: String,
}

#[derive(Parser, Debug)]
struct HistoryCmd {
    #[clap(short, long, default_value = "100")]
    limit: usize,
    #[clap(short, long, default_value = "BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(short, long, default_value = "5m")]
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
struct BalancesCmd {
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
    /// This will return all positions currently held by user.
    Positions(OrderPositionsCmd),
    /// Cancel order with the best effort.
    Cancel(OrderCancelCmd),
}

#[derive(Parser, Debug)]
struct OrderCreateCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(long, default_value = "BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    quantity: u64,
    #[clap(long)]
    price: u64,
    #[clap(long, default_value = "100")]
    leverage: u64,
    #[clap(long)]
    side: OrderSide,
    #[clap(long, default_value = "Isolated")]
    margin_type: MarginType,
    #[clap(long, default_value = "Limit")]
    order_type: OrderType,
    #[clap(long, default_value = "Delayed")]
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
    #[clap(long, default_value = "BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(long, default_value = "100")]
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
    #[clap(long, default_value = "BTCUSD.PERP")]
    symbol: String,
    #[clap(long)]
    start: Option<DateTime<Local>>,
    #[clap(long)]
    end: Option<DateTime<Local>>,
    #[clap(long, default_value = "100")]
    limit: usize,
}

#[derive(Parser, Debug)]
struct OrderPositionsCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
}

#[derive(Parser, Debug)]
struct OrderCancelCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(long, default_value = "BTCUSD.PERP")]
    symbol: String,
    /// ID of order to cancel
    order_id: String,
}

#[derive(Parser, Debug)]
enum WebsocketSub {
    /// Launch websocket with auth info, so account related commands can be executed.
    Private(WebsocketPrivateCmd),
    /// Launch websocket without authentification
    Public(WebsocketPublicCmd),
}

#[derive(Parser, Debug)]
struct WebsocketPrivateCmd {
    #[clap(long, env = "KOLLIDER_API_KEY", hide_env_values = true)]
    api_key: String,
    #[clap(long, env = "KOLLIDER_API_SECRET", hide_env_values = true)]
    api_secret: String,
    #[clap(long, env = "KOLLIDER_API_PASSWORD", hide_env_values = true)]
    password: String,
    /// Which symbol to filter from channels. E.x. '.BTCUSD' or '.BTCEUR'
    #[clap(long)]
    symbols: Vec<Symbol>,
    /// Which channels to listen
    #[clap(long)]
    channels: Vec<ChannelName>,
    #[clap(subcommand)]
    action: Option<WebsocketAction>,
}

#[derive(Parser, Debug)]
enum WebsocketAction {
    Order {
        #[clap(long)]
        price: u64,
        #[clap(long)]
        quantity: u64,
        #[clap(long, default_value = "BTCUSD.PERP")]
        symbol: Symbol,
        #[clap(long, default_value = "100")]
        leverage: u64,
        #[clap(long)]
        side: OrderSide,
        #[clap(long, default_value = "Isolated")]
        margin_type: MarginType,
        #[clap(long, default_value = "Limit")]
        order_type: OrderType,
        #[clap(long, default_value = "Delayed")]
        settlement_type: SettlementType,
    },
    CancelOrder {
        order_id: u64,
        #[clap(long, default_value = "BTCUSD.PERP")]
        symbol: String,
        #[clap(long, default_value = "Delayed")]
        settlement_type: SettlementType,
    },
    FetchOpenOrders,
    FetchPositions,
    FetchBalances,
    GetTicker {
        #[clap(long, default_value = "BTCUSD.PERP")]
        symbol: String,
    },
    TradableProducts,
}

impl WebsocketAction {
    fn to_message(self) -> KolliderMsg {
        match self {
            WebsocketAction::Order {
                price,
                quantity,
                symbol,
                leverage,
                side,
                margin_type,
                order_type,
                settlement_type,
            } => KolliderMsg::Order {
                _type: OrderTag::Tag,
                price,
                quantity,
                symbol,
                leverage,
                side,
                margin_type,
                order_type,
                settlement_type,
                ext_order_id: Uuid::new_v4()
                    .to_hyphenated()
                    .encode_lower(&mut Uuid::encode_buffer())
                    .to_owned(),
            },
            WebsocketAction::CancelOrder {
                order_id,
                symbol,
                settlement_type,
            } => KolliderMsg::CancelOrder {
                _type: CancelOrderTag::Tag,
                order_id,
                symbol,
                settlement_type,
            },
            WebsocketAction::FetchOpenOrders => KolliderMsg::FetchOpenOrders {
                _type: FetchOpenOrdersTag::Tag,
            },
            WebsocketAction::FetchPositions => KolliderMsg::FetchPositions {
                _type: FetchPositionsTag::Tag,
            },
            WebsocketAction::FetchBalances => KolliderMsg::FetchBalances {
                _type: FetchBalancesTag::Tag,
            },
            WebsocketAction::GetTicker { symbol } => KolliderMsg::GetTicker {
                _type: GetTickerTag::Tag,
                symbol,
            },
            WebsocketAction::TradableProducts => KolliderMsg::TradableProducts {
                _type: TradableProductsTag::Tag,
            },
        }
    }
}

#[derive(Parser, Debug)]
struct WebsocketPublicCmd {
    /// Which symbol to filter from channels
    #[clap(long, default_value = "BTCUSD")]
    symbols: Vec<Symbol>,
    /// Which channels to listen
    #[clap(default_value = "index_values")]
    channels: Vec<ChannelName>,
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
        SubCommand::Orderbook(OrderbookCmd { level, symbol }) => {
            let book_level =
                OrderBookLevel::from_int(level).expect("Order book level is either 2 or 3");
            let resp = client.market_orderbook(book_level, &symbol).await?;
            println!("Response /market/orderbook: {:?}", resp);
        }
        SubCommand::Ticker(TickerCmd { symbol }) => {
            let resp = client.market_ticker(&symbol).await?;
            println!("Response /market/ticker: {:?}", resp);
        }
        SubCommand::History(HistoryCmd {
            limit,
            symbol,
            start,
            end,
            interval,
        }) => {
            let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
            let end_time = end.unwrap_or_else(|| Local::now());
            let resp = client
                .market_historic_index_prices(limit, &symbol, start_time, end_time, interval)
                .await?;
            println!("Response /market/historic_index_prices: {:?}", resp);
        }
        SubCommand::Account(AccountCmd {
            api_key,
            api_secret,
            password,
        }) => {
            let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
            client.auth = Some(auth);
            let resp = client.user_account().await?;
            println!("Response /user/account: {:?}", resp);
        }
        SubCommand::Balances(BalancesCmd {
            api_key,
            api_secret,
            password,
        }) => {
            let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
            let resp = fetch_balances(&auth).await?;
            println!("Response WS fetch_balances: {:?}", resp);
        }
        SubCommand::Deposit(ref deposit_sub) => match deposit_sub {
            DepositSub::Btc(DepositBtc {
                api_key,
                api_secret,
                password,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.wallet_deposit(&DepositBody::Bitcoin).await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
            DepositSub::Ln(DepositLn {
                api_key,
                api_secret,
                password,
                amount,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client
                    .wallet_deposit(&DepositBody::Lighting(*amount))
                    .await?;
                println!("Response /wallet/deposit: {:?}", resp);
            }
        },
        SubCommand::Withdrawal(ref withdrawal_sub) => match withdrawal_sub {
            WithdrawalSub::Btc(WithdrawalBtc {
                api_key,
                api_secret,
                password,
                address,
                amount,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client
                    .wallet_withdrawal(&WithdrawalBody::Bitcoin {
                        _type: BtcTag::BTC,
                        receive_address: address.clone(),
                        amount: *amount,
                    })
                    .await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
            WithdrawalSub::Ln(WithdrawalLn {
                api_key,
                api_secret,
                password,
                invoice,
                amount,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client
                    .wallet_withdrawal(&WithdrawalBody::Lighting {
                        _type: LnTag::Ln,
                        payment_request: invoice.clone(),
                        amount: *amount,
                    })
                    .await?;
                println!("Response /wallet/withwallet_withdrawal: {:?}", resp);
            }
        },
        SubCommand::Order(order_sub) => match order_sub {
            OrderSub::Create(OrderCreateCmd {
                api_key,
                api_secret,
                password,
                symbol,
                quantity,
                price,
                leverage,
                side,
                margin_type,
                order_type,
                settlement_type,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client
                    .create_order(&OrderBody {
                        symbol,
                        quantity,
                        price,
                        leverage,
                        side,
                        margin_type,
                        order_type,
                        settlement_type,
                    })
                    .await?;
                println!("Response /orders: {:?}", resp);
            }
            OrderSub::Prediction(OrderCreateCmd {
                api_key,
                api_secret,
                password,
                symbol,
                quantity,
                price,
                leverage,
                side,
                margin_type,
                order_type,
                settlement_type,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client
                    .order_prediction(&OrderBody {
                        symbol,
                        quantity,
                        price,
                        leverage,
                        side,
                        margin_type,
                        order_type,
                        settlement_type,
                    })
                    .await?;
                println!("Response /orders/prediction: {:?}", resp);
            }
            OrderSub::List(OrderListCmd {
                api_key,
                api_secret,
                password,
                symbol,
                start,
                end,
                limit,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
                let end_time = end.unwrap_or_else(|| Local::now());
                let resp = client.orders(&symbol, start_time, end_time, limit).await?;
                println!("Response /orders: {:?}", resp);
            }
            OrderSub::Opened(OrderOpenedCmd {
                api_key,
                api_secret,
                password,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.open_orders().await?;
                println!("Response /orders/opened: {:?}", resp);
            }
            OrderSub::Fills(OrderFillsCmd {
                api_key,
                api_secret,
                password,
                symbol,
                start,
                end,
                limit,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let start_time = start.unwrap_or_else(|| Local::now() - Duration::days(1));
                let end_time = end.unwrap_or_else(|| Local::now());
                let resp = client.fills(&symbol, start_time, end_time, limit).await?;
                println!("Response /user/fills: {:?}", resp);
            }
            OrderSub::Positions(OrderPositionsCmd {
                api_key,
                api_secret,
                password,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.positions().await?;
                println!("Response /positions: {:?}", resp);
            }
            OrderSub::Cancel(OrderCancelCmd {
                api_key,
                api_secret,
                password,
                symbol,
                order_id,
            }) => {
                let auth = KolliderAuth::new(&api_key, &api_secret, &password)?;
                client.auth = Some(auth);
                let resp = client.cancel_order(&symbol, &order_id).await?;
                println!("Response /orders: {:?}", resp);
            }
        },
        SubCommand::Websocket(ws_sub) => match ws_sub {
            WebsocketSub::Private(WebsocketPrivateCmd {
                api_key,
                api_secret,
                password,
                symbols,
                channels,
                action,
            }) => {
                let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
                let (msg_sender, msg_receiver) = futures_channel::mpsc::unbounded();
                let auth_msg = make_user_auth(&api_secret, &api_key, &password)?;
                stdin_tx.unbounded_send(auth_msg)?;
                stdin_tx.unbounded_send(KolliderMsg::Subscribe {
                    _type: SubscribeTag::Tag,
                    channels,
                    symbols,
                })?;
                if let Some(a) = action {
                    stdin_tx.unbounded_send(a.to_message())?;
                }
                // tokio::spawn(websocket_stdin_controller(stdin_tx));
                tokio::spawn(kollider_websocket(stdin_rx, msg_sender));

                msg_receiver
                    .for_each(|message| async move {
                        println!("Received message: {:?}", message);
                    })
                    .await
            }
            WebsocketSub::Public(WebsocketPublicCmd { symbols, channels }) => {
                let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
                let (msg_sender, msg_receiver) = futures_channel::mpsc::unbounded();
                stdin_tx.unbounded_send(KolliderMsg::Subscribe {
                    _type: SubscribeTag::Tag,
                    channels,
                    symbols,
                })?;
                // tokio::spawn(websocket_stdin_controller(stdin_tx));
                tokio::spawn(kollider_websocket(stdin_rx, msg_sender));

                msg_receiver
                    .for_each(|message| async move {
                        println!("Received message: {:?}", message);
                    })
                    .await
            }
        },
    }

    Ok(())
}
