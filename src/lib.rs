#![deny(missing_docs,
  missing_debug_implementations, missing_copy_implementations,
  trivial_casts, trivial_numeric_casts,
  unsafe_code,
  unstable_features,
  unused_import_braces, unused_qualifications)]

//! Archive Bot.

use log::{info, warn};

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

/// Run Archive Bot.
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
  let mut is_member = channel.is_member;
  let ignored = channel_is_ignored(&channel.name, ignore_prefixes);

  if !ignored {
    is_member = maybe_join_channel(&channel, &config.token).await;
  }

  let mut last_message_timestamp: i64 = 0;
  let mut old = false;
  if is_member {
    if let Some(history) = slack_get::get_history(&config.token, &channel.id, 1).await {
      if let Some(latest_message) = history.first() {
        (old, last_message_timestamp) = parse_message(&latest_message, config.stale_after).await;
      }
    }
  }

  let mut small = false;
  let mut num_members = channel.num_members;
  // If in the channel, don't count self.
  if is_member {
    num_members -= 1;
  }
  if num_members <= config.small_channel_threshold as i32 {
    small = true;
  }

  Some(ChannelData {
    id: channel.id,
    name: channel.name,
    last_message: last_message_timestamp,
    members_count: num_members,
    is_old: old,
    is_small: small,
    is_ignored: ignored,
    is_private: channel.is_private,
  })
}

async fn maybe_join_channel(channel: &Channel, token: &str) -> bool {
  let mut is_member = channel.is_member;
  if !is_member && !channel.is_private {
    log::debug!("Need to join channel #{:} ({:})", channel.name, channel.id);
    if false { // TODO: REMOVE ME
      if let Ok(_) = slack_post::join_channel(&token, &channel.id).await {
        is_member = true;
        info!("Joined channel #{:} ({:})", channel.name, channel.id);
      }
    }
  }
  is_member
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

/// Whether the channel is ignored based on config.
fn channel_is_ignored(channel_name: &str, ignore_prefixes: &Vec<&str>) -> bool {
  ignore_prefixes.iter().any(|n| channel_name.starts_with(n))
}

#[cfg(test)]
mod tests {
  #[cfg(any(feature = "unit", feature="unit_output"))]
  use super::*;

  /// Create a test message and print it to stdout rather than posting to Slack.
  #[tokio::test]
  #[cfg(feature = "unit_output")]
  async fn test_create_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
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

  /// Test channel filtering.
  #[tokio::test]
  #[cfg(feature = "unit")]
  async fn test_filter_channels() {
    // (channel name, should be ignored)
    let channels = vec![
      ("testing", false),
      ("-prefixed", true),
      ("ext-another", true),
      ("keep-me", false),
      ("--skip-me", true),
    ];
    let prefixes = vec!["-", "ext-"];
    assert!(channels.iter().any(|(n, r)| channel_is_ignored(n, &prefixes) == *r));
  }

}
