use super::data::*;
use futures_channel::mpsc::UnboundedSender;
use std::str::FromStr;

use shellfish::{Command, Shell};
use std::error::Error;
use std::fmt;

// Our helper method which will read data from stdin and send it along the
// sender provided.
pub async fn websocket_stdin_controller(tx: UnboundedSender<KolliderMsg>) {
    // Define a shell
    let mut shell = Shell::new_async(tx, "$ ");

    // Add some commands
    shell.commands.insert(
        "subscribe",
        Command::new("Subscribe to channel for symbol. Usage: subscribe <symbol> <channel>".to_owned(), subscribe),
    );

    // Run the shell
    shell.run_async().await.unwrap();
}

fn subscribe(tx: &mut UnboundedSender<KolliderMsg>, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let symbol = args.get(1).ok_or_else(|| Box::new(MissingSybmol))?;
    let channel = ChannelName::from_str(args.get(2).ok_or_else(|| Box::new(MissingChannel))?)?;

    let msg = KolliderMsg::Subscribe(SubscribeMsg {
        _type: SubscribeType::Subscribe,
        symbols: vec![symbol.clone()],
        channels: vec![channel.clone()],
    });
    tx.unbounded_send(msg)?;
    Ok(())
}

#[derive(Debug)]
struct MissingSybmol;

impl fmt::Display for MissingSybmol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No symbol specified: BTCUSD.PERP")
    }
}

impl Error for MissingSybmol {}

#[derive(Debug)]
struct MissingChannel;

impl fmt::Display for MissingChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No channel name specified: index_values, orderbook_level1, orderbook_level2, orderbook_level3, ticker, matches")
    }
}

impl Error for MissingChannel {}