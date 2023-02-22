use crate::ArchiveBot;
use crate::types::*;
use crate::error::SlackError;

impl ArchiveBot {
  /// Wrapper for reqwest client to make Slack API calls.
  pub async fn slack_query<'sq>(&self, method: &str, params: &mut UrlParams<'sq>) -> Result<String, reqwest::Error> {
    let mut p: UrlParams = vec![("token", self.token.to_owned())];
    p.append(params);

    let url = format!("https://slack.com/api/{}", method);
    let client = reqwest::Client::new();
    Ok(client.post(url).form(&p).send().await?.text().await?)
  }

  /// Send specific API call and parse response.
  pub async fn send<'sq>(&self, method: &str, params: &mut UrlParams<'sq>) -> Result<SlackResponse, SlackError<reqwest::Error>> {
    let response = self.slack_query(method, params).await;

    response.map_err(SlackError::Client)
      .and_then(|result| {
        serde_json::from_str::<SlackResponse>(&result)
          .map_err(|e| SlackError::MalformedResponse(result, e))
      })
      .and_then(|o| o.into())
  }
}

#[cfg(test)]
mod tests {
  #[cfg(feature = "unit")]
  #[tokio::test]
  async fn test_auth() {
    let bot = crate::ArchiveBot::_from_env_debug();
    let mut params: crate::types::UrlParams = vec![];
    if let Ok(auth) = bot.send("auth.test", &mut params).await {
      if let Some(user) = auth.user {
        assert!(user != "");
      }
    }
  }
}
