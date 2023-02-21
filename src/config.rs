use std::env;

/// Config for Archive Bot.
#[derive(Debug)]
pub struct Config<'cfg> {
  /// Slack bot token.
  pub token: String,
  /// Channel id to send notifications.
  pub notification_channel_id: String,
  /// Vector of channel prefixes to filter out of results.
  pub filter_prefixes: Vec<&'cfg str>,
  /// Vector of messages to send (one at random) at beginning of updates.
  pub message_headers: Vec<&'cfg str>,
  /// How long until a channel is stale (in seconds).
  pub stale_after: u32,
  /// The threshold <= channels are considered "small".
  pub small_channel_threshold: u16,
  /// Whether to notify a secondary channel of updates (such as #general).
  pub notify_secondary_channel: bool,
  /// Secondary channel id.
  pub secondary_notification_channel_id: String,
  /// Secondary notification message options.
  pub secondary_message_headers: Vec<&'cfg str>,
}

impl<'cfg> Default for Config<'cfg> {
  /// To append to configuration to fill blank values with defaults.
  fn default() -> Config<'cfg> {
    Config {
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

impl<'cfg> Config<'cfg> {
  /// Create a configuration from environment variables.
  pub fn from_env() -> Config<'cfg> {
    Config {
      token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
      notification_channel_id: env::var("SLACK_CHANNEL_ID").expect("Error: environment variable SLACK_CHANNEL_ID is not set."),
      ..Config::default()
    }
  }

  /// Create a configuration from environment variables for debug purposes.
  pub fn from_env_debug() -> Config<'cfg> {
    Config {
      token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
      notification_channel_id: env::var("SLACK_CHANNEL_ID").expect("Error: environment variable SLACK_CHANNEL_ID is not set."),
      filter_prefixes: vec!["-", "ext-"],
      notify_secondary_channel: true,
      secondary_notification_channel_id: env::var("SLACK_CHANNEL2_ID").expect("Error: environment variable SLACK_CHANNEL2_ID is not set."),
      ..Config::default()
    }
  }
}

/// How many messages to pull from a channel to recent activity.
pub const MESSAGE_HISTORY_LENGTH: u16 = 10;
