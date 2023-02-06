use super::slack_client;
use crate::types::{UrlParams, Channel, Message, SlackError};

pub async fn get_channels(key: &str) -> Option<Vec<Channel>> {
  let params: UrlParams = vec![
    ("token", key),
    ("exclude_archived", "1"),
    ("exclude_members", "0"),
    ("limit", "1000"),
    ("types", "public_channel,private_channel"),
  ];

  let response = slack_client::send("conversations.list", &params).await;

  match response {
    Ok(list) => {return list.channels},
    Err(err) => panic!("Error: {}", err),
  }
}

pub async fn get_history(key: &str, channel_id: &str, limit: u16) -> Option<Vec<Message>> {
  let limit: &str = &limit.to_string()[..];
  let params: UrlParams = vec![
    ("token", key),
    ("channel", &channel_id),
    ("limit", &limit),
  ];

  let response: Result<crate::types::SlackResponse, crate::types::SlackError<reqwest::Error>> = slack_client::send("conversations.history", &params).await;

  match response {
    Ok(list) => {return list.messages},
    Err(err) => match err {
      SlackError::NotInChannel => { println!("Not in channel: {:}", channel_id); None },
      _ => panic!("Error: {}", err),
    }
  }
}

#[allow(dead_code)]
pub async fn message_is_old(message: &Message) -> bool {
  match &message.ts {
    Some(ts) => {
      let now = chrono::offset::Utc::now().timestamp();
      // if the message is older than two weeks
      if *ts < (now - (2 * 7 * 24 * 60 * 60)) {
        return true;
      }
    },
    _ => {}
  }

  false
}
