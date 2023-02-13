use std::env;

pub struct Config<'cfg> {
  pub token: String,
  pub notification_channel_id: String,
  pub filter_prefixes: Vec<&'cfg str>,
  pub message_headers: Vec<&'cfg str>,
  pub stale_after: u32,
  pub small_channel_threshold: u16,
}

impl<'cfg> Default for Config<'cfg> {
  fn default() -> Config<'cfg> {
    Config {
      token: "".to_string(),
      notification_channel_id: "".to_string(),
      filter_prefixes: vec![],
      message_headers: vec![
        "Hey, you've got some cleaning up to do!",
        "Hey boss, take a look at these, will ya?",
      ],
      stale_after: 2 * 7 * 24 * 60 * 60,
      small_channel_threshold: 3,
    }
  }
}

impl<'cfg> Config<'cfg> {
  pub fn from_env() -> Config<'cfg> {
    Config {
      token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
      notification_channel_id: env::var("SLACK_CHANNEL_ID").expect("Error: environment variable SLACK_CHANNEL_ID is not set."),
      filter_prefixes: vec![],
      message_headers: vec![
        "Hey, you've got some cleaning up to do!",
        "Hey boss, take a look at these, will ya?",
      ],
      stale_after: 2 * 7 * 24 * 60 * 60,
      small_channel_threshold: 3,
    }
  }
}
