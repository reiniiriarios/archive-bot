use super::slack_client;
use crate::types::{UrlParams, SlackResponse, SlackError};

pub async fn post_message(token: &str, channel_id: &str, message: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let params: UrlParams = vec![
    ("token", token),
    ("channel", channel_id),
    ("text", message),
    ("mrkdwn", "1"),
  ];

  slack_client::send("chat.postMessage", &params).await
}

pub async fn join_channel(token: &str, channel_id: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let params: UrlParams = vec![
    ("token", token),
    ("channel", channel_id),
  ];

  slack_client::send("conversations.join", &params).await
}
