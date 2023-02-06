use std::env;
use substring::Substring;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set.");
  // TODO move to config
  let prefixes = vec!["-"];
  if let Some(channels) = channels::get_channels(&api_key).await {
    for channel in channels {
      parse_channel(&api_key, channel, &prefixes).await;
    }
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
    if is_member {
      if let Some(history) = channels::get_history(&api_key, &channel_id, 1).await {
        println!("  message: {:?}", history.first());
      }
    }
  }
}
