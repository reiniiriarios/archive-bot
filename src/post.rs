use log::warn;

use crate::ArchiveBot;
use crate::types::*;
use crate::error::SlackError;

impl ArchiveBot {
  /// Post a message to a channel.
  pub async fn post_message(&self, channel_id: &str, message: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
    let mut params: UrlParams = vec![
      ("channel", channel_id.to_string()),
      ("text", message.to_string()),
      ("mrkdwn", String::from("1")),
    ];

    match self.send("chat.postMessage", &mut params).await {
      Ok(r) => Ok(r),
      Err(e) => {
        warn!("Unable to post message: {:}", e);
        Err(e)
      },
    }
  }

  /// Make Archive Bot join a channel.
  pub async fn join_channel(&self, channel_id: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
    let mut params: UrlParams = vec![
      ("channel", channel_id.to_string()),
    ];

    match self.send("conversations.join", &mut params).await {
      Ok(r) => Ok(r),
      Err(e) => {
        warn!("Unable to join channel: {:}", e);
        Err(e)
      },
    }
  }
}

#[cfg(test)]
mod tests {
  #[cfg(feature="unit_output")]
  use std::env;
  #[cfg(feature="unit_output")]
  use log::{info, error};
  #[cfg(feature="unit_output")]
  use simplelog;

  /// Create a test message and print it to stdout rather than posting to Slack.
  #[tokio::test]
  #[cfg(feature = "unit_output")]
  async fn test_send_message() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let bot = crate::ArchiveBot::_from_env_debug();
    let channel_id = env::var("SLACK_CHANNEL_TEST_ID").expect("Error: environment variable SLACK_CHANNEL_TEST_ID is not set.");
    let message = "Testing, 1 2 3. :robot_face:";
    match bot.post_message(&channel_id, message).await {
      Ok(_) => info!("Message sent."),
      Err(e) => error!("Error: {}", e),
    }
  }
}
