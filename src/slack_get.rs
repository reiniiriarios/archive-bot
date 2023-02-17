use super::slack_client;
use crate::types::UrlParams;
use crate::slack_error::SlackError;
use crate::slack_response::{SlackResponse, Channel, Message};
use log::{debug, warn};

/// Get a comprehensive list of basic channel data.
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
  debug!("{} channels found", channels.len());

  channels
}

/// Get channel data for given cursor (pagination).
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
      let cursor = match resp.response_metadata {
        Some(metadata) => metadata.next_cursor,
        None => "".into(),
      };
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

/// Get conversation history for.
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

#[cfg(test)]
mod tests {
  #[cfg(feature="unit_output")]
  use super::*;
  #[cfg(feature="unit_output")]
  use simplelog;

  /// Create a test message and print it to stdout rather than posting to Slack.
  #[tokio::test]
  #[cfg(feature = "unit_output")]
  async fn test_list_channels() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let config = crate::config::Config::from_env_debug();

    let mut channel_names: String = String::from("");
    for channel in get_channels(&config.token).await {
      let check = { if channel.is_member { "✅" } else { "❌" } };
      channel_names.push_str(&format!("{} #{}\n", check, channel.name).to_owned());
    }
    println!("Channels:\n{:}", channel_names);
  }
}
