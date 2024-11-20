mod proto;
mod util;

use std::{env, fs};
use anyhow::Result;
use async_nats::Client;
use protobuf::{Message, MessageField};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use tracing::info;
use uuid::Uuid;
use crate::proto::stats_update::UpdateStats;
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
    playtime: i32,
    #[serde(rename = "minecraft:deaths")]
    deaths: Option<i32>,
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

fn get_arg(index: usize) -> Option<String> {
    env::args().nth(index)
}

#[tokio::main]
async fn main() -> Result<()> {

    let name = match get_arg(1) {
        Some(arg) => arg,
        None => {
            println!("No first argument provided");
            //return Err(anyhow::anyhow!("No first argument provided"));
            return Ok(());
        },
    };

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
    let raw_uuids = list_json_files("./stats")?;

    for raw_uuid in raw_uuids {
        // get uuid.
        let uuid = Uuid::parse_str(raw_uuid.as_str())?;

        // parse json
        let (playtime, deaths) = parse_json_file(&*("./stats/".to_owned() + &*raw_uuid + ".json"))?;

        // debug message
        println!("{} - Deaths: {} Playtime: {}",
                 uuid.to_string(),
                 deaths,
                 playtime);

        // build stats object.
        let mut stats = proto::stats::Stats::new();
        stats.deaths = Some(deaths);
        stats.playtime = Some(playtime);
        stats.minecraft_uuid = uuid.to_string();
        stats.server = name.clone();

        // send message
        let mut msg = UpdateStats::new();
        msg.stats = MessageField::some(stats);
    }

    Ok(())
}

/// Convert the json file into deaths and playtime.
///
/// # Arguments
/// * `file_path` - Path to the file to parse.
///
/// # Returns
/// * Tuple of playtime, deaths
fn parse_json_file(file_path: &str) -> anyhow::Result<(i32, i32)> {
    // Read the file contents
    let data = fs::read_to_string(file_path)?;

    // Parse the JSON string into the struct
    let parsed: Wrapper = serde_json::from_str(&data)?;

    Ok((parsed.stats.minecraft_custom.playtime, parsed.stats.minecraft_custom.deaths.unwrap_or(0)))
}

/// Get a list of json file names.
///
/// # Arguments
/// * `dir_path` - The directory to look for files in
///
/// # Returns
/// * A list of file names without the .json.
pub fn list_json_files(dir_path: &str) -> anyhow::Result<Vec<String>> {
    let mut json_files = Vec::new();

    // Read directory entries
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if it's a file and has .json extension
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    if let Some(file_name) = path.file_stem() {
                        if let Some(name_str) = file_name.to_str() {
                            json_files.push(name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(json_files)
}
