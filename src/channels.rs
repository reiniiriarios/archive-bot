use super::slack_client;
use crate::types::{UrlParams, Channel, Message};

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

#[allow(dead_code)]
pub async fn channel_is_old(channel: &Channel) -> bool {
  match &channel.latest {
    Some(Message {
        ts: Some(ts),
        ..
    }) => {
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
