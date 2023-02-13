use std::{error::Error, fmt};

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
