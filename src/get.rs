use log::{debug, warn};

use crate::ArchiveBot;
use crate::types::*;
use crate::error::SlackError;

impl ArchiveBot {
  /// Get a comprehensive list of basic channel data.
  pub async fn get_channels(&self) -> Vec<Channel> {
    let mut channels: Vec<Channel> = vec![];
    let mut cursor: String = "".to_string();
    loop {
      let (more_channels, next_cursor) = self.get_channel_data(cursor).await;
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
  async fn get_channel_data(&self, cursor: String) -> (Vec<Channel>, String) {
    let mut params: UrlParams = vec![
      ("exclude_archived", String::from("1")),
      ("exclude_members", String::from("0")),
      ("limit", String::from("1000")),
      ("types", String::from("public_channel,private_channel")),
    ];
    if cursor != "" {
      params.push(("cursor", cursor));
    }

    let response = self.send("conversations.list", &mut params).await;

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
  pub async fn get_history(&self, channel_id: &str, limit: u16) -> Option<Vec<Message>> {
    let mut params: UrlParams = vec![
      ("channel", channel_id.to_string()),
      ("limit", limit.to_string()),
    ];

    let response: Result<SlackResponse, SlackError<reqwest::Error>> = self.send("conversations.history", &mut params).await;

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
}

#[cfg(test)]
mod tests {
  #[cfg(feature="unit_output")]
  use simplelog;

  /// Create a test message and print it to stdout rather than posting to Slack.
  #[tokio::test]
  #[cfg(feature = "unit_output")]
  async fn test_list_channels() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let bot = crate::ArchiveBot::_from_env_debug();

    let mut channel_names: String = String::from("");
    for channel in bot.get_channels().await {
      let check = { if channel.is_member { "✅" } else { "❌" } };
      channel_names.push_str(&format!("{} #{}\n", check, channel.name).to_owned());
    }
    println!("Channels:\n{:}", channel_names);
  }
}
