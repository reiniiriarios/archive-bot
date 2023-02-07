use std::env;
use substring::Substring;

use crate::channels::message_is_old;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = types::Config {
    api_key: env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set."),
    filter_prefixes: vec!["-"],
  };
  run(&config).await;

  Ok(())
}

async fn run(config: &types::Config) {
  let mut channels_data: Vec<types::ChannelData> = vec![];
  for channel in channels::get_channels(&config.api_key).await {
    if let Some(channel_data) = parse_channel(&config.api_key, channel, &config.filter_prefixes).await {
      channels_data.push(channel_data);
    }
  }
  // TODO parse channels_data
}

async fn parse_channel(api_key: &str, channel: types::Channel, ignore_prefixes: &Vec<&str>) -> Option<types::ChannelData> {
  if let (
    Some(channel_id),
    Some(channel_name),
    Some(is_member),
  ) = (
    channel.id,
    channel.name,
    channel.is_member,
  ) {
    println!("Channel: #{:}", channel_name);
    if ignore_prefixes.contains(&channel_name.substring(0, 1)) {
      println!("  prefix skipped");
      return None;
    }

    // TODO join channel
    if !is_member {
      join_channel(&channel_id).await;
    }

    let mut last_message_timestamp: i64 = 0;
    let mut old = false;
    if is_member {
      if let Some(history) = channels::get_history(&api_key, &channel_id, 1).await {
        if let Some(latest_message) = history.first() {
          (old, last_message_timestamp) = parse_message(&latest_message).await;
        }
      }
    }

    let num_members = 0;
    let mut small = false;
    if let Some(num_members) = channel.num_members {
      if num_members < 4 {
        small = true;
        println!("  only {:} members", num_members);
      }
      else {
        println!("  there are {:} members", num_members);
      }
    }

    return Some(types::ChannelData {
      name: channel_name,
      last_message: last_message_timestamp,
      members_count: num_members,
      is_old: old,
      is_small: small,
    });
  }
  None
}

async fn parse_message(message: &types::Message) -> (bool, i64) {
  let mut t: i64 = 0;
  if let Some(ts) = message.ts {
    t = ts.into();
  }
  let mut old = false;
  if message_is_old(&message) {
    println!("  message is old");
    old = true;
  }
  else {
    println!("  message is recent");
  }

  (old, t)
}

async fn join_channel(channel_id: &str) {
  println!("  TODO join channel {:}", channel_id);
}
