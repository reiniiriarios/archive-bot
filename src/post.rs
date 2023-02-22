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
