use slack_api;

use crate::config::Config;

pub async fn get_channels(cfg: &Config) -> Option<Vec<slack_api::Channel>> {
  let client = slack_api::default_client().unwrap();
  let params = Default::default();
  let response = slack_api::channels::list(&client, &cfg.slack_api_key, &params).await;
  match response {
    Ok(list) => {return list.channels},
    Err(err) => panic!("Error: {}", err),
  }
}

pub async fn channel_is_old(channel: &slack_api::Channel) -> bool {
  match &channel.latest {
    Some(slack_api::Message::Standard(slack_api::MessageStandard {
        ts: Some(ts),
        ..
    })) => {
      let time: i64 = ts.to_param_value().parse().unwrap();
      let now = chrono::offset::Utc::now().timestamp();
      // if the message is older than two weeks
      if time < (now - (2 * 7 * 24 * 60 * 60)) {
        return true;
      }
    },
    _ => {}
  }

  false
}
