use super::slack_client;
use crate::types::{UrlParams, SlackResponse, SlackError};
use log::error;

pub async fn post_message(token: &str, channel_id: &str, message: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let params: UrlParams = vec![
    ("token", token),
    ("channel", channel_id),
    ("text", message),
    ("mrkdwn", "1"),
  ];

  match slack_client::send("chat.postMessage", &params).await {
    Ok(r) => Ok(r),
    Err(e) => {
      error!("Error: {:}", e);
      Err(e)
    },
  }
}

pub async fn join_channel(token: &str, channel_id: &str) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let params: UrlParams = vec![
    ("token", token),
    ("channel", channel_id),
  ];

  match slack_client::send("conversations.join", &params).await {
    Ok(r) => Ok(r),
    Err(e) => {
      error!("Error: {:}", e);
      Err(e)
    },
  }
}
