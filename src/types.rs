use std::{error::Error, default::Default, fmt};
use serde::Deserialize;

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
  InvalidAuth,
  AccessDenied,
  AuthTimeout,
  AuthVerification,
  ChannelNotFound,
  NotInChannel,
  IsArchived,
  InvalidScopes,
  CommentRequired,
  RateLimited,
  InvalidCursor,
  InvalidLimit,
  InvalidType,
  Fatal,
  Internal,
  MalformedResponse(String, serde_json::error::Error),
  Unknown(String),
  Client(E),
}

impl<'a, E: Error> From<&'a str> for SlackError<E> {
  fn from(s: &'a str) -> Self {
    match s {
      "invalid_auth" => SlackError::InvalidAuth,
      "access_denied" => SlackError::AccessDenied,
      "auth_timeout_error" => SlackError::AuthTimeout,
      "auth_verification_error" => SlackError::AuthVerification,
      "channel_not_found" => SlackError::ChannelNotFound,
      "not_in_channel" => SlackError::NotInChannel,
      "is_archived" => SlackError::IsArchived,
      "invalid_scopes" => SlackError::InvalidScopes,
      "comment_required" => SlackError::CommentRequired,
      "ratelimited" => SlackError::RateLimited,
      "rate_limited" => SlackError::RateLimited,
      "invalid_cursor" => SlackError::InvalidCursor,
      "invalid_limit" => SlackError::InvalidLimit,
      "invalid_types" => SlackError::InvalidType,
      "fatal_error" => SlackError::Fatal,
      "internal_error" => SlackError::Internal,
      _ => SlackError::Unknown(s.to_owned()),
    }
  }
}

impl<E: Error> fmt::Display for SlackError<E> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let d = match *self {
      SlackError::InvalidAuth => "Invalid authentication token.",
      SlackError::AccessDenied => "You don't have permissions to create Slack-hosted apps or access the specified resource.",
      SlackError::AuthTimeout => "Couldn't receive authorization in the time allowed.",
      SlackError::AuthVerification => "Couldn't verify your authorization.",
      SlackError::ChannelNotFound => "Couldn't find the specified Slack channel.",
      SlackError::NotInChannel => "Cannot post user messages to a channel they are not in.",
      SlackError::IsArchived => "Channel has been archived.",
      SlackError::InvalidScopes => "Some of the provided scopes do not exist.",
      SlackError::CommentRequired => "Your App Manager is requesting a reason to approve installation of this app.",
      SlackError::RateLimited => "Too many calls in succession to create endpoint during a short period of time.",
      SlackError::InvalidCursor => "Value passed for `cursor` was not valid or is no longer valid.",
      SlackError::InvalidLimit => "Value passed for `limit` is not understood.",
      SlackError::InvalidType => "Value passed for `type` could not be used based on the method's capabilities or the permission scopes granted to the used token.",
      SlackError::Fatal => "The server could not complete your operation(s) without encountering a catastrophic error.",
      SlackError::Internal => "The server could not complete your operation(s) without encountering an error, likely due to a transient issue with Slack.",
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
