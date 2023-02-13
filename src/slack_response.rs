use std::{error::Error, default::Default};
use serde::Deserialize;
use crate::slack_error::SlackError;

/// Possible json response from Slack API.
#[derive(Clone, Debug, Deserialize)]
pub struct SlackResponse {
  pub channels: Option<Vec<Channel>>,
  pub messages: Option<Vec<Message>>,
  pub url: Option<String>,
  pub team: Option<String>,
  pub user: Option<String>,
  pub team_id: Option<String>,
  pub user_id: Option<String>,
  pub bot_id: Option<String>,
  pub response_metadata: Option<SlackResponseMeta>,
  error: Option<String>,
  #[serde(default)]
  ok: bool,
}

/// Metadata including next cursor (pagination marker).
#[derive(Clone, Debug, Deserialize)]
pub struct SlackResponseMeta {
  pub next_cursor: Option<String>,
}

impl<E: Error> Into<Result<SlackResponse, SlackError<E>>> for SlackResponse {
  /// Parse response.
  fn into(self) -> Result<SlackResponse, SlackError<E>> {
    if self.ok {
      Ok(self)
    } else {
      Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
    }
  }
}

/// Conversation response. Non-comprehensive.
/// https://api.slack.com/types/conversation
#[derive(Clone, Debug, Deserialize)]
pub struct Channel {
  pub id: Option<String>,
  pub name: Option<String>,
  pub is_channel: Option<bool>,
  pub is_group: Option<bool>,
  pub is_im: Option<bool>,
  pub created: Option<Timestamp>,
  pub creator: Option<String>,
  pub is_archived: Option<bool>,
  pub is_general: Option<bool>,
  pub unlinked: Option<bool>,
  pub name_normalized: Option<String>,
  pub is_read_only: Option<bool>,
  pub is_shared: Option<bool>,
  pub is_ext_shared: Option<bool>,
  pub is_org_shared: Option<bool>,
  pub pending_shared: Option<Vec<String>>,
  pub is_pending_ext_shared: Option<bool>,
  pub is_member: Option<bool>,
  pub is_private: Option<bool>,
  pub is_mpim: Option<bool>,
  pub last_read: Option<String>,
  // topic
  // purpose
  pub previous_names: Option<Vec<String>>,
  pub num_members: Option<i32>,
}

/// Message data response. Non-comprehensive.
/// https://api.slack.com/events/message
#[derive(Clone, Debug, Deserialize)]
pub struct Message {
  #[serde(rename = "type")]
  pub event_type: Option<String>,
  pub subtype: Option<String>,
  pub channel: Option<String>,
  pub user: Option<String>,
  pub text: Option<String>,
  pub ts: Option<Timestamp>,
  // edited
}

/// Timestamp type for Slack responses.
/// Slack returns a f64 in a string that cannot be directly parsed.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Timestamp(i64);

impl Timestamp {
  pub fn new(t: i64) -> Timestamp {
    Timestamp(t)
  }
}

impl From<f64> for Timestamp {
  fn from(t: f64) -> Self {
    Timestamp(t as i64)
  }
}

impl From<i64> for Timestamp {
  fn from(t: i64) -> Self {
    Timestamp(t)
  }
}

impl From<u64> for Timestamp {
  fn from(t: u64) -> Self {
    Timestamp(t as i64)
  }
}

impl From<Timestamp> for i64 {
  fn from(t: Timestamp) -> Self {
    t.0
  }
}

impl<'de> ::serde::Deserialize<'de> for Timestamp {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
    use serde::de::Error as SerdeError;
    let value = ::serde_json::Value::deserialize(deserializer)?;

    // Possible formats of timestamp from slack response are u64, f64,
    // or str as: "", "1234567890", or "1234567890.1234567890".
    // Unable to directly parse the float strings as i64.
    if let Some(s) = value.as_str() {
      if let Some(dot_index) = s.find('.') {
        if let Ok(i) = s[..dot_index].parse::<i64>() {
          return Ok(i.into());
        }
      } else if let Ok(u) = s.parse::<i64>() {
        return Ok(u.into());
      }
    } else if let Some(f) = value.as_f64() {
      return Ok(f.into());
    } else if let Some(u) = value.as_u64() {
      return Ok((u as f64).into());
    }

    Err(D::Error::custom(format!(
      "expected a timestamp but got: {}",
      value.to_string()
    )))
  }
}
