mod proto;
mod util;

use anyhow::Result;
use async_nats::Client;
use protobuf::Message;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use tracing::info;

/*#[tracing::instrument]
async fn send_hello(nc: async_nats::Client, from: &str) -> Result<()> {

    // create test message
    let mut msg = Hello::new();
    msg.from = from.to_string();

    // Serialize the user to bytes
    let encoded: Vec<u8> = msg.write_to_bytes().unwrap();

    // send message
    let publisher_client = nc.clone();
    publisher_client.publish("hello", encoded.into()).await?;

    Ok(())
}*/

#[derive(Deserialize)]
struct Custom {
    #[serde(rename = "minecraft:play_time")]
    playtime: i64,
    #[serde(rename = "minecraft:deaths")]
    deaths: i32,
}

#[derive(Deserialize)]
struct Stats {
    #[serde(rename = "minecraft:custom")]
    minecraft_custom: Custom,
}

#[derive(Deserialize)]
struct Wrapper {
    stats: Stats,
}

#[tokio::main]
async fn main() -> Result<()> {

    // get the app name, used for group and such
    let app_name = match util::get_app_name() {
        Some(name) => name,
        None => { return Err(anyhow::anyhow!("Could not  determine application name.")); },
    };

    // Setup logging
    util::setup_logging(app_name.as_str());

    // connect to nats
    let nc = util::connect_to_nats().await?;

    // find each *.json file in current directory

    // parse uuid from file name

    // find deaths and playtime

    // build message and send.

    Ok(())
}
