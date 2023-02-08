use crate::types::{UrlParams, SlackResponse, SlackError};

pub async fn slack_query<'sq>(method: &str, params: &'sq UrlParams<'sq>) -> Result<String, reqwest::Error> {
  let url = format!("https://slack.com/api/{}", method);
  let client = reqwest::Client::new();
  Ok(client.post(url).form(params).send().await?.text().await?)
}

pub async fn send<'sq>(method: &str, params: &'sq UrlParams<'sq>) -> Result<SlackResponse, SlackError<reqwest::Error>> {
  let response = slack_query(method, params).await;

  response.map_err(SlackError::Client)
    .and_then(|result| {
      serde_json::from_str::<SlackResponse>(&result)
        .map_err(|e| SlackError::MalformedResponse(result, e))
    })
    .and_then(|o| o.into())
}

#[cfg(test)]
mod tests {
  use std::env;
  use super::*;

  #[tokio::test]
  async fn test_auth() {
    let token = env::var("SLACK_BOT_TOKEN").expect("Error: environment variable SLACK_BOT_TOKEN is not set.");
    let params: UrlParams = vec![("token", &token)];
    if let Ok(auth) = send("auth.test", &params).await {
      if let Some(user) = auth.user {
        assert!(user != "");
      }
    }
  }
}
