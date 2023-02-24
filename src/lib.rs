#![deny(missing_docs,
  missing_debug_implementations, missing_copy_implementations,
  trivial_casts, trivial_numeric_casts,
  unsafe_code,
  unstable_features,
  unused_import_braces, unused_qualifications)]

//! Archive Bot.

use std::env;
use log::info;
use rand::seq::SliceRandom;
use chrono;
use futures::future;

mod client;
mod error;
mod get;
mod post;
mod types;

use types::*;

/// How many messages to pull from a channel to recent activity.
const MESSAGE_HISTORY_LENGTH: u16 = 10;

/// Archive bot.
#[derive(Debug)]
pub struct ArchiveBot {
  /// Slack bot token.
  pub token: String,
  /// Channel id to send notifications.
  pub notification_channel_id: String,
  /// Vector of channel prefixes to filter out of results.
  pub filter_prefixes: Vec<&'static str>,
  /// Vector of messages to send (one at random) at beginning of updates.
  pub message_headers: Vec<&'static str>,
  /// How long until a channel is stale (in seconds).
  pub stale_after: u32,
  /// The threshold <= channels are considered "small".
  pub small_channel_threshold: u16,
  /// Whether to notify a secondary channel of updates (such as #general).
  pub notify_secondary_channel: bool,
  /// Secondary channel id.
  pub secondary_notification_channel_id: String,
  /// Secondary notification message options.
  pub secondary_message_headers: Vec<&'static str>,
}

impl Default for ArchiveBot {
  /// To append to configuration to fill blank values with defaults.
  fn default() -> ArchiveBot {
    ArchiveBot {
      token: "".to_string(),
      notification_channel_id: "".to_string(),
      filter_prefixes: vec![],
      message_headers: vec![
        "Hey, you've got some cleaning up to do!",
        "Hey boss, take a look at these, will ya?",
        "I don't know what this is, or what to do with it:",
      ],
      stale_after: 6 * 7 * 24 * 60 * 60,
      small_channel_threshold: 3,
      notify_secondary_channel: false,
      secondary_notification_channel_id: "".to_string(),
      secondary_message_headers: vec![
        "Hey folks! I, uh... made a list for you. Of channels. That you should archive. Maybe.",
        "Hey everyone! If you want the satisfaction of crossing a task off your list, I have one!",
        "BEEP, BOOP! Archival update: List generated. End of program."
      ],
    }
  }
}

impl ArchiveBot {
  /// Create a configuration from environment variables.
  pub fn from_env() -> ArchiveBot {
    ArchiveBot {
      token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
      notification_channel_id: env::var("SLACK_CHANNEL_ID").expect("Error: environment variable SLACK_CHANNEL_ID is not set."),
      ..ArchiveBot::default()
    }
  }

  /// Create a configuration from environment variables for debug purposes.
  fn _from_env_debug() -> ArchiveBot {
    let test_channel = env::var("SLACK_CHANNEL_TEST_ID").expect("Error: environment variable SLACK_CHANNEL_TEST_ID is not set.");
    ArchiveBot {
      token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
      notification_channel_id: test_channel.clone(),
      filter_prefixes: vec!["-", "ext-"],
      notify_secondary_channel: true,
      secondary_notification_channel_id: test_channel,
      ..ArchiveBot::default()
    }
  }

  /// Run Archive Bot.
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Get channels.
    let channels = self.get_channels().await;

    // Parse each channel concurrently.
    let mut tasks = vec![];
    for channel in &channels {
      tasks.push(self.parse_channel(&channel));
    }
    let channels_data = future::join_all(tasks).await;

    // Build and send message.
    let message = self.create_message(&channels_data);
    if message != "" {
      if let Ok(_) = self.post_message(&self.notification_channel_id, &message).await {
        info!("Posted update in {:}", self.notification_channel_id);
        if self.notify_secondary_channel {
          let secondary_message = self.create_secondary_message();
          if let Ok(_) = self.post_message(&self.secondary_notification_channel_id, &secondary_message).await {
            info!("Posted secondary update in {:}", self.secondary_notification_channel_id);
          }
        }
      }
    }

    Ok(())
  }

  /// Parse data to create regular update message to post regarding channel status.
  fn create_message(&self, data: &Vec<ChannelData>) -> String {
    let mut message: String = "".to_string();
    for channel in data {
      if self.channel_should_be_mentioned(&channel) {
        let mbr_msg: String = match channel {
          ChannelData { is_small: true, .. } => format!("has *{} members*.", channel.num_members),
          _ => format!("has {} members.", channel.num_members),
        };

        let time_msg: String = match channel {
          ChannelData { is_private: true, .. } => "The channel is private, so I can't read the latest message.".into(),
          ChannelData { last_message_ts: 0, .. } => "No recent messages.".into(),
          ChannelData { last_message_relevant: false, is_old: true, .. } => format!("The last event was on *{date}*, but there are no recent messages.", date=channel.last_message_ts_formatted()),
          ChannelData { last_message_relevant: false, is_old: false, .. } => format!("The last event was on {date}, but there are no recent messages.", date=channel.last_message_ts_formatted()),
          ChannelData { is_old: true, .. } => format!("The last message was on *{date}*.", date=channel.last_message_ts_formatted()),
          _ => format!("The last message was on {date}.", date=channel.last_message_ts_formatted()),
        };

        // mrkdwn parsed, but no list format; using * breaks *bold* text
        message.push_str(&format!(
          "- <#{id}> {members} {time}\n",
          id=channel.id,
          members=mbr_msg,
          time=time_msg
        ));
      }
    }

    if message != "" {
      let prefix = self.message_headers.choose(&mut rand::thread_rng()).unwrap().to_string();
      message = format!("{}\n{}", prefix, message);
    }

    message
  }

  /// Whether a channel should be included in updates.
  fn channel_should_be_mentioned(&self, channel: &ChannelData) -> bool {
    (channel.is_old || channel.is_small) && !channel.is_ignored
  }

  /// Create secondary notification message.
  fn create_secondary_message(&self) -> String {
    let line_a = self.secondary_message_headers.choose(&mut rand::thread_rng()).unwrap().to_string();
    format!("{} See <#{}> for details.", line_a, self.notification_channel_id)
  }

  /// Parse a specific channel for relevant data, fetching missing data where necessary.
  async fn parse_channel(&self, channel: &Channel) -> ChannelData {
    let is_ignored = self.channel_is_ignored(&channel.name);

    let is_member = match is_ignored {
      true => channel.is_member,
      false => self.maybe_join_channel(&channel).await,
    };

    let mut last_message_ts = 0;
    let mut last_message_relevant = false;
    let mut is_old = false;

    if is_member && !is_ignored {
      if let Some(msg) = self.get_last_message(&channel).await {
        if let Some(ts) = msg.ts {
          last_message_ts = ts.into();
        }
        last_message_relevant = !msg.ignore_type();
        let now = chrono::offset::Utc::now().timestamp();
        is_old = last_message_ts > 0 && last_message_ts < now - self.stale_after as i64;
      };
    }

    // Don't count self as a member.
    let num_members = match is_member {
      true => channel.num_members - 1,
      false => channel.num_members,
    };
    let is_small = num_members <= self.small_channel_threshold as i32;

    ChannelData {
      id: channel.id.clone(),
      name: channel.name.clone(),
      last_message_ts,
      last_message_relevant,
      num_members,
      is_old,
      is_small,
      is_ignored,
      is_private: channel.is_private,
    }
  }

  /// Join a channel (maybe). Returns whether the bot is now a member of the channel.
  async fn maybe_join_channel(&self, channel: &Channel) -> bool {
    if !channel.is_member && !channel.is_private {
      log::debug!("Need to join channel #{:} ({:})", channel.name, channel.id);
      if let Ok(_) = self.join_channel(&channel.id).await {
        info!("Joined channel #{:} ({:})", channel.name, channel.id);
        return true;
      }
    }
    channel.is_member
  }

  /// Get timestamp of last message in a channel.
  async fn get_last_message(&self, channel: &Channel) -> Option<Message> {
    if let Some(history) = self.get_history(&channel.id, MESSAGE_HISTORY_LENGTH).await {
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
  fn channel_is_ignored(&self, channel_name: &str) -> bool {
    self.filter_prefixes.iter().any(|n| channel_name.starts_with(n))
  }
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
    let bot = crate::ArchiveBot::_from_env_debug();

    let mut channels_data: Vec<ChannelData> = vec![];
    for channel in bot.get_channels().await {
      channels_data.push(bot.parse_channel(&channel).await);
    }
    let message = bot.create_message(&channels_data);
    println!("Message:\n{:}", message);
  }

  /// Create a test secondary message and print it to stdout rather than posting to Slack.
  #[test]
  #[cfg(feature = "unit_output")]
  fn test_create_secondary_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let bot = crate::ArchiveBot::_from_env_debug();
    let message = bot.create_secondary_message();
    println!("Message:\n{:}", message);
  }

  /// Test channel filtering.
  #[tokio::test]
  #[cfg(feature = "unit")]
  async fn test_filter_channels() {
    let bot = crate::ArchiveBot::_from_env_debug();
    // (channel name, should be ignored)
    let channels = vec![
      ("testing", false),
      ("-prefixed", true),
      ("ext-another", true),
      ("keep-me", false),
      ("--skip-me", true),
    ];
    assert!(channels.iter().any(|(n, r)| bot.channel_is_ignored(n) == *r));
  }

  /// Test channel parsing.
  #[tokio::test]
  #[cfg(feature = "unit")]
  async fn test_parse_channel() {
    use crate::types::ChannelData;

    let bot = crate::ArchiveBot::_from_env_debug();

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

    let data = bot.parse_channel(&channel).await;
    assert_eq!(data, test_channel_data);
  }

}
