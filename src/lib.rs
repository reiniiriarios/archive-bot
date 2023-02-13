use log::{info, warn};

use substring::Substring;
use rand::seq::SliceRandom;

mod slack_client;
mod slack_get;
mod slack_post;
mod slack_error;
mod slack_response;
use slack_response::*;
mod types;
use types::*;
mod config;
pub use self::config::Config;

pub async fn run<'cfg>(config: &Config<'cfg>) -> Result<(), Box<dyn std::error::Error>> {
  let mut channels_data: Vec<ChannelData> = vec![];
  for channel in slack_get::get_channels(&config.token).await {
    if let Some(channel_data) = parse_channel(&config, channel, &config.filter_prefixes).await {
      channels_data.push(channel_data);
    }
  }
  let message = create_message(&config, &channels_data);
  if message != "" {
    let post = slack_post::post_message(&config.token, &config.notification_channel_id, &message).await;
    if let Ok(_) = post {
      info!("Posted update in {:}", config.notification_channel_id);
    }
  }

  Ok(())
}

/// Parse data to create regular update message to post regarding channel status.
fn create_message<'cfg>(config: &Config<'cfg>, data: &Vec<ChannelData>) -> String {
  let mut message: String = "".to_string();
  for channel in data {
    if (channel.is_old || channel.is_small) && !channel.is_ignored {
      let line: String = {
        if channel.last_message == 0 {
          warn!("Unable to parse timestamp for channel #{:} ({:})", channel.name, channel.id);
          format!(
            "* <#{id}> has {members} members. I'm having trouble reading the latest message.\n",
            id=channel.id,
            members=channel.members_count,
          )
        }
        else {
          format!(
            "* <#{id}> has {members} members. The latest message was on {date}.\n",
            id=channel.id,
            members=channel.members_count,
            date=channel.last_message_formatted()
          )
        }
      };
      message.push_str(&line);
    }
  }

  if message != "" {
    let prefix = config.message_headers.choose(&mut rand::thread_rng()).unwrap().to_string();
    message = format!("{}\n{}", prefix, message);
  }

  message
}

/// Parse a specific channel for relevant data, fetching missing data where necessary.
async fn parse_channel<'cfg>(config: &Config<'cfg>, channel: Channel, ignore_prefixes: &Vec<&str>) -> Option<ChannelData> {
  if let (
    Some(channel_id),
    Some(channel_name),
    Some(mut is_member),
  ) = (
    channel.id,
    channel.name,
    channel.is_member,
  ) {
    let ignored = ignore_prefixes.contains(&channel_name.substring(0, 1));

    // TODO join channel
    if !is_member && false {
      if let Ok(_) = slack_post::join_channel(&config.token, &channel_id).await {
        is_member = true;
        info!("Joined channel #{:} ({:})", channel_name, channel_id);
      }
    }

    let mut last_message_timestamp: i64 = 0;
    let mut old = false;
    if is_member {
      if let Some(history) = slack_get::get_history(&config.token, &channel_id, 1).await {
        if let Some(latest_message) = history.first() {
          (old, last_message_timestamp) = parse_message(&latest_message, config.stale_after).await;
        }
      }
    }

    let mut num_members = 0;
    let mut small = false;
    if let Some(c_num_members) = channel.num_members {
      num_members = c_num_members;
      if num_members < config.small_channel_threshold as i32 {
        small = true;
      }
    }

    return Some(ChannelData {
      id: channel_id,
      name: channel_name,
      last_message: last_message_timestamp,
      members_count: num_members,
      is_old: old,
      is_small: small,
      is_ignored: ignored,
    });
  }

  None
}

/// Parse a message to get `is_old` status and timestamp of message.
async fn parse_message(message: &Message, stale_after: u32) -> (bool, i64) {
  let mut t: i64 = 0;
  if let Some(ts) = message.ts {
    t = ts.into();
  }
  let mut old = false;
  if let Some(ts) = message.ts {
    let now = chrono::offset::Utc::now().timestamp();
    if ts < Timestamp::new(now - stale_after as i64) {
      old = true;
    }
  }

  (old, t)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_create_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let config = Config::from_env();

    let mut channels_data: Vec<ChannelData> = vec![];
    for channel in slack_get::get_channels(&config.token).await {
      if let Some(channel_data) = parse_channel(&config, channel, &config.filter_prefixes).await {
        channels_data.push(channel_data);
      }
    }
    let message = create_message(&config, &channels_data);
    println!("Message:\n{:}", message);
  }
}
