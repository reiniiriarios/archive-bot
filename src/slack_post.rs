use super::slack_client;
use crate::types::{UrlParams, SlackResponse, SlackError};

pub async fn post_message(api_key: &str, channel_id: &str, message: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let params: UrlParams = vec![
    ("token", api_key),
    ("channel", channel_id),
    ("text", message),
    ("mrkdwn", "1"),
  ];

  slack_client::send("chat.postMessage", &params).await
}
