use crate::types::UrlParams;
use crate::slack_error::SlackError;
use crate::slack_response::SlackResponse;

/// Wrapper for reqwest client to make Slack API calls.
pub async fn slack_query<'sq>(method: &str, params: &'sq UrlParams<'sq>) -> Result<String, reqwest::Error> {
  let url = format!("https://slack.com/api/{}", method);
  let client = reqwest::Client::new();
  Ok(client.post(url).form(params).send().await?.text().await?)
}

/// Send specific API call and parse response.
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
  #[cfg(feature = "unit")]
  use crate::Config;
  #[cfg(feature = "unit")]
  use super::*;

  #[cfg(feature = "unit")]
  #[tokio::test]
  async fn test_auth() {
    let config = Config::from_env_debug();
    let params: UrlParams = vec![("token", &config.token)];
    if let Ok(auth) = send("auth.test", &params).await {
      if let Some(user) = auth.user {
        assert!(user != "");
      }
    }
  }
}
