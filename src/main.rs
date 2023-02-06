use slack_client::channel_is_old;

mod config;
mod slack_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cfg: config::Config = config::get_config();
  let channels = slack_client::get_channels(&cfg).await;
  match channels {
    Some(channels) => {
      for channel in channels {
        if channel_is_old(&channel).await {

        }
      }
    },
    None => {},
  }

  Ok(())
}
