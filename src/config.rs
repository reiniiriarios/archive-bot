pub struct Config {
  pub token: String,
  pub notification_channel_id: &'static str,
  pub filter_prefixes: Vec<&'static str>,
  pub message_headers: Vec<&'static str>,
  pub stale_after: u32,
  pub small_channel_threshold: u16,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      token: "".to_string(),
      notification_channel_id: "",
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
