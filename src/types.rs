use std::{error::Error, fmt};
use serde::Deserialize;

pub type UrlParams<'sq> = Vec<(&'sq str, &'sq str)>;

#[derive(Clone, Debug, Deserialize)]
pub struct SlackResponse {
  pub channels: Option<Vec<Channel>>,
  error: Option<String>,
  #[serde(default)]
  ok: bool,
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
  pub created: Option<i32>,
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
  pub latest: Option<Message>,
  pub members: Option<Vec<String>>,
  pub name: Option<String>,
  pub name_normalized: Option<String>,
  pub num_members: Option<i32>,
  pub previous_names: Option<Vec<String>>,
  pub priority: Option<f32>,
  pub unlinked: Option<i32>,
  pub unread_count: Option<i32>,
  pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
  pub bot_id: Option<String>,
  pub bot_link: Option<String>,
  pub channel: Option<String>,
  pub subtype: Option<String>,
  pub team: Option<String>,
  pub source_team: Option<String>,
  pub text: Option<String>,
  pub reply_broadcast: Option<bool>,
  pub ts: Option<i64>,
  pub thread_ts: Option<i64>,
  pub event_ts: Option<i64>,
  pub deleted_ts: Option<i64>,
  #[serde(rename = "type")]
  pub ty: Option<String>,
  pub user: Option<String>,
  pub username: Option<String>,
  pub members: Option<Vec<String>>,
  pub inviter: Option<String>,
  pub old_name: Option<String>,
  pub purpose: Option<String>,
  pub topic: Option<String>,
  pub upload: Option<bool>,
  pub hidden: Option<bool>,
  pub last_read: Option<String>,
  pub parent_user_id: Option<String>,
  pub reply_count: Option<i32>,
  pub subscribed: Option<bool>,
  pub unread_count: Option<i32>,
  pub item_type: Option<String>,
}
