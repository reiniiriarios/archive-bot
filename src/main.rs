use std::env;
use substring::Substring;

use crate::channels::message_is_old;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set.");
  // TODO move to config
  let prefixes = vec!["-"];
  for channel in channels::get_channels(&api_key).await {
    parse_channel(&api_key, channel, &prefixes).await;
  }

  Ok(())
}

async fn parse_channel(api_key: &str, channel: types::Channel, ignore_prefixes: &Vec<&str>) {
  if let (
    Some(channel_id),
    Some(name),
    Some(is_member),
  ) = (
    channel.id,
    channel.name,
    channel.is_member,
  ) {
    println!("Channel: #{:}", name);
    if ignore_prefixes.contains(&name.substring(0, 1)) {
      println!("  prefix skipped");
      return;
    }

    // TODO join channel
    if !is_member {
      println!("  TODO join channel");
    }

    let mut old = false;
    if is_member {
      if let Some(history) = channels::get_history(&api_key, &channel_id, 1).await {
        if let Some(latest_message) = history.first() {
          if message_is_old(&latest_message) {
            println!("  message is old");
            old = true;
          }
          else {
            println!("  message is recent");
          }
        }
      }
    }

    if !old {
      if let Some(num_members) = channel.num_members {
        if num_members < 4 {
          println!("  only {:} members", num_members);
        }
        else {
          println!("  there are {:} members", num_members);
        }
      }
    }
  }
}
