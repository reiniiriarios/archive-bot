use std::env;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set.");
  if let Some(channels) = channels::get_channels(&api_key).await {
    for channel in channels {
      if let (
        Some(channel_id),
        Some(name),
        Some(is_member),
      ) = (
        channel.id,
        channel.name,
        channel.is_member,
      ) {
        println!("Channel: #{:}", name);
        if is_member {
          if let Some(history) = channels::get_history(&api_key, &channel_id, 1).await {
            println!("  message: {:?}", history.first());
          }
        }
      }
    }
  }

  Ok(())
}
