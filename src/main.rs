use std::env;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set.");
  let _ = slack_client::auth_test(&api_key);
  if let Some(channels) = channels::get_channels(&api_key).await {
    for channel in channels {
      println!("channel: {:?}", channel.name);
      if let (Some(channel_id), Some(is_member)) = (channel.id, channel.is_member) {
        if is_member {
          if let Some(history) = channels::get_history(&api_key, &channel_id, 1).await {
            println!("message: {:?}", history.first());
          }
        }
      }
    }
  }

  Ok(())
}
