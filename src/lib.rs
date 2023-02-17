#![deny(missing_docs,
  missing_debug_implementations, missing_copy_implementations,
  trivial_casts, trivial_numeric_casts,
  unsafe_code,
  unstable_features,
  unused_import_braces, unused_qualifications)]

//! Archive Bot.

use config::MESSAGE_HISTORY_LENGTH;
use log::info;
use rand::seq::SliceRandom;
use chrono;

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
    if let Some(channel_data) = parse_channel(&config, channel).await {
      channels_data.push(channel_data);
    }
  }
  let message = create_message(&config, &channels_data);
  if message != "" {
    if let Ok(_) = slack_post::post_message(&config.token, &config.notification_channel_id, &message).await {
      info!("Posted update in {:}", config.notification_channel_id);
      if config.notify_secondary_channel {
        let secondary_message = create_secondary_message(&config);
        if let Ok(_) = slack_post::post_message(&config.token, &config.secondary_notification_channel_id, &secondary_message).await {
          info!("Posted secondary update in {:}", config.secondary_notification_channel_id);
        }
      }
    }
  }

  Ok(())
}

/// Parse data to create regular update message to post regarding channel status.
fn create_message<'cfg>(config: &Config<'cfg>, data: &Vec<ChannelData>) -> String {
  let mut message: String = "".to_string();
  for channel in data {
    if (channel.is_old || channel.is_small) && !channel.is_ignored {
      let time_msg: String = match channel {
        ChannelData { is_private: true, .. } => "The channel is private, so I can't read the latest message.".into(),
        ChannelData { last_message_ts: 0, .. } => "No recent messages.".into(),
        ChannelData { last_message_relevant: false, .. } => format!("The last event was on {date}, but there are no recent messages.", date=channel.last_message_ts_formatted()),
        _ => format!("The last message was on {date}.", date=channel.last_message_ts_formatted()),
      };
      let line = format!(
        "* <#{id}> has {members} members. {time_msg}\n",
        id=channel.id,
        members=channel.num_members,
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

/// Create secondary notification message.
fn create_secondary_message<'cfg>(config: &Config<'cfg>) -> String {
  let line_a = config.secondary_message_headers.choose(&mut rand::thread_rng()).unwrap().to_string();
  format!("{} See <#{}> for details.", line_a, config.notification_channel_id)
}

/// Parse a specific channel for relevant data, fetching missing data where necessary.
async fn parse_channel<'cfg>(config: &Config<'cfg>, channel: Channel) -> Option<ChannelData> {
  let is_ignored = channel_is_ignored(&channel.name, &config.filter_prefixes);

  let is_member = match is_ignored {
    true => channel.is_member,
    false => maybe_join_channel(&channel, &config.token).await,
  };

  let mut last_message_ts = 0;
  let mut last_message_relevant = false;
  let mut is_old = false;

  if is_member && !is_ignored {
    (last_message_ts, last_message_relevant, is_old) = match get_last_message(&channel, &config.token).await {
      Some(msg) => {
        let last_message_ts = match msg.ts {
          Some(ts) => ts.into(),
          None => 0,
        };
        let now = chrono::offset::Utc::now().timestamp();
        let is_old = last_message_ts > 0 && last_message_ts < now - config.stale_after as i64;
        (last_message_ts, !msg.ignore_type(), is_old)
      },
      None => (0, false, true),
    };
  }

  // Don't count self as a member.
  let num_members = match is_member {
    true => channel.num_members - 1,
    false => channel.num_members,
  };
  let is_small = num_members <= config.small_channel_threshold as i32;

  Some(ChannelData {
    id: channel.id,
    name: channel.name,
    last_message_ts,
    last_message_relevant,
    num_members,
    is_old,
    is_small,
    is_ignored,
    is_private: channel.is_private,
  })
}

/// Join a channel (maybe). Returns whether the bot is now a member of the channel.
async fn maybe_join_channel(channel: &Channel, token: &str) -> bool {
  if !channel.is_member && !channel.is_private {
    log::debug!("Need to join channel #{:} ({:})", channel.name, channel.id);
    if let Ok(_) = slack_post::join_channel(&token, &channel.id).await {
      info!("Joined channel #{:} ({:})", channel.name, channel.id);
      return true;
    }
  }
  channel.is_member
}

/// Get timestamp of last message in a channel.
async fn get_last_message(channel: &Channel, token: &str) -> Option<Message> {
  if let Some(history) = slack_get::get_history(&token, &channel.id, MESSAGE_HISTORY_LENGTH).await {
    for message in history.clone() {
      if !message.ignore_type() {
        if let Some(_ts) = message.ts {
          return Some(message);
        }
      }
    }
    if let Some(first) = history.first() {
      return Some(first.to_owned());
    }
  }
  None
}

/// Whether the channel is ignored based on config.
fn channel_is_ignored(channel_name: &str, ignore_prefixes: &Vec<&str>) -> bool {
  ignore_prefixes.iter().any(|n| channel_name.starts_with(n))
}

#[cfg(test)]
mod tests {
  #[cfg(any(feature = "unit", feature="unit_output"))]
  use super::*;
  #[cfg(feature="unit_output")]
  use simplelog;

  /// Create a test message and print it to stdout rather than posting to Slack.
  #[tokio::test]
  #[cfg(feature = "unit_output")]
  async fn test_create_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let config = Config::from_env_debug();

    let mut channels_data: Vec<ChannelData> = vec![];
    for channel in slack_get::get_channels(&config.token).await {
      if let Some(channel_data) = parse_channel(&config, channel).await {
        channels_data.push(channel_data);
      }
    }
    let message = create_message(&config, &channels_data);
    println!("Message:\n{:}", message);
  }

  /// Create a test secondary message and print it to stdout rather than posting to Slack.
  #[test]
  #[cfg(feature = "unit_output")]
  fn test_create_secondary_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let config = Config::from_env_debug();
    let message = create_secondary_message(&config);
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

  /// Test channel parsing.
  #[tokio::test]
  #[cfg(feature = "unit")]
  async fn test_parse_channel() {
    use crate::types::ChannelData;

    let config = Config::from_env_debug();

    let channel: Channel = Channel {
      id: "fake_id".to_string(),
      name: "fake-name".to_string(),
      is_channel: true,
      is_group: false,
      is_im: false,
      is_archived: false,
      is_general: false,
      unlinked: false,
      is_read_only: false,
      is_shared: false,
      is_ext_shared: false,
      is_org_shared: false,
      is_pending_ext_shared: false,
      is_member: false,
      is_private: true,
      is_mpim: false,
      num_members: 3,
      creator: None,
      created: None,
      last_read: None,
      name_normalized: None,
      pending_shared: None,
      previous_names: None,
    };

    let test_channel_data: ChannelData = ChannelData {
      id: "fake_id".to_string(),
      name: "fake-name".to_string(),
      last_message_ts: 0,
      last_message_relevant: false,
      num_members: 3,
      is_old: false,
      is_small: true,
      is_ignored: false,
      is_private: true,
    };

    match parse_channel(&config, channel).await {
      Some(channel_data) => assert_eq!(channel_data, test_channel_data),
      None => assert!(false),
    }
  }

}
