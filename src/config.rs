use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub slack_api_key: String,
  pub message: String,
}

impl ::std::default::Default for Config {
  fn default() -> Self { Self { slack_api_key: "".into(), message: "".into() } }
}

pub fn get_config() -> Config {
  let cfg = confy::load("archive-bot", None);
  match cfg {
    Ok(config) => {return config},
    Err(err) => panic!("Error: {}", err),
  }
}
