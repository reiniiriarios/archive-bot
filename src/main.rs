use std::env;
use substring::Substring;
use rand::seq::SliceRandom;
use chrono::NaiveDateTime;

mod slack_client;
mod slack_get;
mod slack_post;
mod types;
use types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = types::Config {
    api_key: env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set."),
    notification_channel_id: "C04N8B12VDK",
    filter_prefixes: vec!["-"],
    message_headers: vec![
      "Hey, you've got some cleaning up to do!",
      "Hey boss, take a look at these, will ya?",
    ],
    stale_after: 2 * 7 * 24 * 60 * 60,
    small_channel_threshold: 3,
  };
  run(&config).await;

  Ok(())
}

async fn run(config: &Config) {
  let mut channels_data: Vec<ChannelData> = vec![];
  for channel in slack_get::get_channels(&config.api_key).await {
    if let Some(channel_data) = parse_channel(&config, channel, &config.filter_prefixes).await {
      channels_data.push(channel_data);
    }
  }
  let message = create_message(&config, &channels_data);
  if message != "" {
    let post = slack_post::post_message(&config.api_key, &config.notification_channel_id, &message).await;
    if let Err(e) = post {
      println!("Error: {:}", e);
    }
  }
}

fn create_message(config: &Config, data: &Vec<ChannelData>) -> String {
  let mut message: String = "".to_string();
  for channel in data {
    if (channel.is_old || channel.is_small) && !channel.is_ignored {
      let line: String = {
        if channel.last_message == 0 {
          format!(
            "* <#{id}> has {members} members. I'm having trouble reading the latest message.\n",
            id=channel.id,
            members=channel.members_count,
          )
        }
        else {
          format!(
            "* <#{id}> has {members} members. The latest message was on {date}.\n",
            id=channel.id,
            members=channel.members_count,
            date=format_timestamp(channel.last_message)
          )
        }
      };
      message.push_str(&line);
    }
  }

  if message != "" {
    let prefix = config.message_headers.choose(&mut rand::thread_rng()).unwrap().to_string();
    message = format!("{}\n{}", prefix, message);
  }

  message
}

fn format_timestamp(t: i64) -> String {
  if t == 0 { return "[unable to parse timestamp]".to_string() }
  NaiveDateTime::from_timestamp_opt(t, 0).unwrap().format("%b %d, %Y").to_string()
}

async fn parse_channel(config: &Config, channel: Channel, ignore_prefixes: &Vec<&str>) -> Option<ChannelData> {
  if let (
    Some(channel_id),
    Some(channel_name),
    Some(is_member),
  ) = (
    channel.id,
    channel.name,
    channel.is_member,
  ) {
    let ignored = ignore_prefixes.contains(&channel_name.substring(0, 1));

    // TODO join channel
    if !is_member {
      join_channel(&channel_id).await;
    }

    let mut last_message_timestamp: i64 = 0;
    let mut old = false;
    if is_member {
      if let Some(history) = slack_get::get_history(&config.api_key, &channel_id, 1).await {
        if let Some(latest_message) = history.first() {
          (old, last_message_timestamp) = parse_message(&latest_message, config.stale_after).await;
        }
      }
    }

    let mut num_members = 0;
    let mut small = false;
    if let Some(c_num_members) = channel.num_members {
      num_members = c_num_members;
      if num_members < config.small_channel_threshold as i32 {
        small = true;
      }
    }

    return Some(types::ChannelData {
      id: channel_id,
      name: channel_name,
      last_message: last_message_timestamp,
      members_count: num_members,
      is_old: old,
      is_small: small,
      is_ignored: ignored,
    });
  }

  None
}

async fn parse_message(message: &types::Message, stale_after: u32) -> (bool, i64) {
  let mut t: i64 = 0;
  if let Some(ts) = message.ts {
    t = ts.into();
  }
  let mut old = false;
  if let Some(ts) = message.ts {
    let now = chrono::offset::Utc::now().timestamp();
    if ts < crate::types::Timestamp::new(now - stale_after as i64) {
      old = true;
    }
  }

  (old, t)
}

#[allow(unused_variables)]
async fn join_channel(channel_id: &str) {
  // TODO join channel
}
