pub struct Config<'cfg> {
  pub token: String,
  pub notification_channel_id: &'cfg str,
  pub filter_prefixes: Vec<&'cfg str>,
  pub message_headers: Vec<&'cfg str>,
  pub stale_after: u32,
  pub small_channel_threshold: u16,
}

impl<'cfg> Default for Config<'cfg> {
  fn default() -> Config<'cfg> {
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
