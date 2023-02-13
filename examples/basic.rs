use std::env;
use archive_bot;

#[tokio::main]
async fn main() {
  let config = archive_bot::Config {
    token: env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set."),
    notification_channel_id: "A01A02A03A04".to_string(),
    filter_prefixes: vec!["-"],
    message_headers: vec![
      "Hey, you've got some cleaning up to do!",
      "Hey boss, take a look at these, will ya?",
    ],
    stale_after: 2 * 7 * 24 * 60 * 60,
    small_channel_threshold: 3,
    ..archive_bot::Config::default()
  };

  match archive_bot::run(&config).await {
    Ok(_) => println!("Success!"),
    Err(e) => panic!("Uhoh! {:}", e),
  }
}
