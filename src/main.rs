use std::env;

mod channels;
mod slack_client;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = env::var("SLACK_API_KEY").expect("Error: environment variable SLACK_API_KEY is not set.");
  let _ = slack_client::auth_test(&api_key);
  let channels = channels::get_channels(&api_key).await;
  match channels {
    Some(channels) => {
      for channel in channels {
        println!("channel: {:?}", channel.name);
        println!("{:?}", channel.created);
      }
    },
    None => {},
  }

  Ok(())
}
