use super::slack_client;
use crate::types::UrlParams;
use crate::slack_error::SlackError;
use crate::slack_response::{SlackResponse, Channel, Message};
use log::warn;

pub async fn get_channels<'sq>(token: &str) -> Vec<Channel> {
  let mut channels: Vec<Channel> = vec![];
  let mut cursor: String = "".to_string();
  loop {
    let (more_channels, next_cursor) = get_channel_data(&token, cursor).await;
    channels.extend(more_channels);
    if next_cursor == "" {
      break;
    }
    cursor = next_cursor;
  }
  channels
}

async fn get_channel_data<'sq>(token: &str, cursor: String) -> (Vec<Channel>, String) {
  let mut params: UrlParams = vec![
    ("token", token),
    ("exclude_archived", "1"),
    ("exclude_members", "0"),
    ("limit", "1000"),
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
    Err(err) => {
      warn!("Unable to list channels: {:}", err);
    },
  }

  (vec![], "".to_string())
}

pub async fn get_history(token: &str, channel_id: &str, limit: u16) -> Option<Vec<Message>> {
  let limit: &str = &limit.to_string()[..];
  let params: UrlParams = vec![
    ("token", token),
    ("channel", &channel_id),
    ("limit", &limit),
  ];

  let response: Result<SlackResponse, SlackError<reqwest::Error>> = slack_client::send("conversations.history", &params).await;

  match response {
    Ok(list) => {return list.messages},
    Err(err) => match err {
      SlackError::NotInChannel => {
        warn!("Not in channel {:}", channel_id);
        None
      },
      _ => {
        warn!("Unable to fetch channel history: {:}", err);
        None
      },
    }
  }
}
