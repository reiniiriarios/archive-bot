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
      let time_msg: String = match (channel.last_message, channel.is_private) {
        (0, true) => "The channel is private, so I can't read the latest message.".into(),
        (0, false) => {
          warn!("Unable to parse timestamp for channel #{:} ({:})", channel.name, channel.id);
          "I'm having trouble reading the latest message.".into()
        },
        _ => format!("The latest message was on {date}.", date=channel.last_message_formatted()),
      };
      let line = format!(
        "* <#{id}> has {members} members. {time_msg}\n",
        id=channel.id,
        members=channel.members_count,
      );
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
  let ignored = channel_is_ignored(&channel.name, ignore_prefixes);

  let is_member = match ignored {
    true => channel.is_member,
    false => maybe_join_channel(&channel, &config.token).await,
  };

  let last_message_timestamp = match is_member {
    true => get_last_message_timestamp(&channel, &config.token).await,
    false => 0,
  };
  let now = chrono::offset::Utc::now().timestamp();
  let old = last_message_timestamp > 0 && last_message_timestamp < now - config.stale_after as i64;

  // Don't count self as a member.
  let num_members = match is_member {
    true => channel.num_members - 1,
    false => channel.num_members,
  };
  let small = num_members <= config.small_channel_threshold as i32;

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

/// Join a channel (maybe). Returns whether the bot is now a member of the channel.
async fn maybe_join_channel(channel: &Channel, token: &str) -> bool {
  if !channel.is_member && !channel.is_private {
    log::debug!("Need to join channel #{:} ({:})", channel.name, channel.id);
    if false { // TODO: REMOVE ME
      if let Ok(_) = slack_post::join_channel(&token, &channel.id).await {
        info!("Joined channel #{:} ({:})", channel.name, channel.id);
        return true;
      }
    }
  }
  channel.is_member
}

/// Get timestamp of last message in a channel.
async fn get_last_message_timestamp(channel: &Channel, token: &str) -> i64 {
  if let Some(history) = slack_get::get_history(&token, &channel.id, 1).await {
    if let Some(latest_message) = history.first() {
      if let Some(ts) = latest_message.ts {
        return ts.into()
      }
    }
  }
  0
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
