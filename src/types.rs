use std::{error::Error, default::Default, fmt};
use serde::Deserialize;

pub struct Config {
  pub token: String,
  pub notification_channel_id: &'static str,
  pub filter_prefixes: Vec<&'static str>,
  pub message_headers: Vec<&'static str>,
  pub stale_after: u32,
  pub small_channel_threshold: u16,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      token: "".to_string(),
      notification_channel_id: "",
      filter_prefixes: vec![],
      message_headers: vec![
        "Hey, you've got some cleaning up to do!",
        "Hey boss, take a look at these, will ya?",
      ],
      stale_after: 2 * 7 * 24 * 60 * 60,
      small_channel_threshold: 3,
    }
  }
}

pub type UrlParams<'sq> = Vec<(&'sq str, &'sq str)>;

pub struct ChannelData {
  pub id: String,
  pub name: String,
  pub last_message: i64,
  pub members_count: i32,
  pub is_old: bool,
  pub is_small: bool,
  pub is_ignored: bool,
}

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

#[derive(Clone, Debug, Deserialize)]
pub struct SlackResponseMeta {
  pub next_cursor: Option<String>,
}

impl<E: Error> Into<Result<SlackResponse, SlackError<E>>> for SlackResponse {
  fn into(self) -> Result<SlackResponse, SlackError<E>> {
    if self.ok {
      Ok(self)
    } else {
      Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
    }
  }
}

#[derive(Debug)]
pub enum SlackError<E: Error> {
  NotAuthed,
  InvalidAuth,
  MissingScope,
  NotInChannel,
  AccountInactive,
  InvalidArgName,
  InvalidArrayArg,
  InvalidCharset,
  InvalidFormData,
  InvalidPostType,
  MissingPostType,
  TeamAddedToOrg,
  RequestTimeout,
  MalformedResponse(String, serde_json::error::Error),
  Unknown(String),
  Client(E),
}

impl<'a, E: Error> From<&'a str> for SlackError<E> {
  fn from(s: &'a str) -> Self {
    match s {
      "not_authed" => SlackError::NotAuthed,
      "invalid_auth" => SlackError::InvalidAuth,
      "missing_scope" => SlackError::MissingScope,
      "not_in_channel" => SlackError::NotInChannel,
      "account_inactive" => SlackError::AccountInactive,
      "invalid_arg_name" => SlackError::InvalidArgName,
      "invalid_array_arg" => SlackError::InvalidArrayArg,
      "invalid_charset" => SlackError::InvalidCharset,
      "invalid_form_data" => SlackError::InvalidFormData,
      "invalid_post_type" => SlackError::InvalidPostType,
      "missing_post_type" => SlackError::MissingPostType,
      "team_added_to_org" => SlackError::TeamAddedToOrg,
      "request_timeout" => SlackError::RequestTimeout,
      _ => SlackError::Unknown(s.to_owned()),
    }
  }
}

impl<E: Error> fmt::Display for SlackError<E> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let d = match *self {
      SlackError::NotAuthed => "No authentication token provided.",
      SlackError::InvalidAuth => "Invalid authentication token.",
      SlackError::MissingScope => "Missing permissions scope.",
      SlackError::NotInChannel => "Not in channel.",
      SlackError::AccountInactive => "Authentication token is for a deleted user or team.",
      SlackError::InvalidArgName => "Invalid argument.",
      SlackError::InvalidArrayArg => "Invalid argument, in form of array.",
      SlackError::InvalidCharset => "Invalid charset.", // Valid sets are: utf-8, iso-8859-1.
      SlackError::InvalidFormData => "Invalid form data.",
      SlackError::InvalidPostType => "Invalid post type.", // Valid types are: application/x-www-form-urlencoded, multipart/form-data, text/plain.
      SlackError::MissingPostType => "Missing Content-Type header.",
      SlackError::TeamAddedToOrg => "Team temporarily inaccessible.",
      SlackError::RequestTimeout => "Request timeout.",
      SlackError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
      SlackError::Unknown(ref s) => return write!(f, "{}", s),
      SlackError::Client(ref inner) => return write!(f, "{}", inner),
    };
    write!(f, "{}", d)
  }
}

impl<E: Error + 'static> Error for SlackError<E> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      SlackError::MalformedResponse(_, ref e) => Some(e),
      SlackError::Client(ref inner) => Some(inner),
      _ => None,
    }
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Channel {
  pub accepted_user: Option<String>,
  pub created: Option<Timestamp>,
  pub creator: Option<String>,
  pub id: Option<String>,
  pub is_archived: Option<bool>,
  pub is_channel: Option<bool>,
  pub is_general: Option<bool>,
  pub is_member: Option<bool>,
  pub is_moved: Option<i32>,
  pub is_mpim: Option<bool>,
  pub is_org_shared: Option<bool>,
  pub is_pending_ext_shared: Option<bool>,
  pub is_private: Option<bool>,
  pub is_read_only: Option<bool>,
  pub is_shared: Option<bool>,
  pub last_read: Option<String>,
  pub messages: Option<Vec<Message>>,
  pub members: Option<Vec<String>>,
  pub name: Option<String>,
  pub name_normalized: Option<String>,
  pub num_members: Option<i32>,
  pub previous_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
  pub text: Option<String>,
  pub ts: Option<Timestamp>,
  #[serde(rename = "type")]
  pub ty: Option<String>,
  pub user: Option<String>,
}

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
