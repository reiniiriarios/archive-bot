use super::slack_client;
use crate::types::{UrlParams, Channel, Message, SlackResponse, SlackError};

pub async fn get_channels<'sq>(key: &str) -> Vec<Channel> {
  let mut channels: Vec<Channel> = vec![];
  let mut cursor: String = "".to_string();
  loop {
    let (more_channels, next_cursor) = get_channel_data(&key, cursor).await;
    channels.extend(more_channels);
    if next_cursor == "" {
      break;
    }
    cursor = next_cursor;
  }
  channels
}

async fn get_channel_data<'sq>(key: &str, cursor: String) -> (Vec<Channel>, String) {
  let mut params: UrlParams = vec![
    ("token", key),
    ("exclude_archived", "1"),
    ("exclude_members", "0"),
    ("limit", "100"),
    ("types", "public_channel,private_channel"),
  ];
  if cursor != "" {
    params.push(("cursor", &cursor));
  }

  let response = slack_client::send("conversations.list", &params).await;

  match response {
    Ok(resp) => {
      let mut cursor: String = "".to_string();
      if let Some(metadata) = resp.response_metadata {
        if let Some(c) = metadata.next_cursor {
          cursor = c;
        }
      }
      if let Some(channels) = resp.channels {
        return (channels, cursor);
      }
    },
    Err(err) => panic!("Error: {}", err),
  }

  (vec![], "".to_string())
}

pub async fn get_history(key: &str, channel_id: &str, limit: u16) -> Option<Vec<Message>> {
  let limit: &str = &limit.to_string()[..];
  let params: UrlParams = vec![
    ("token", key),
    ("channel", &channel_id),
    ("limit", &limit),
  ];

  let response: Result<SlackResponse, SlackError<reqwest::Error>> = slack_client::send("conversations.history", &params).await;

  match response {
    Ok(list) => {return list.messages},
    Err(err) => match err {
      SlackError::NotInChannel => { println!("Not in channel: {:}", channel_id); None },
      _ => panic!("Error: {}", err),
    }
  }
}
