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

pub async fn auth_test(key: &str) {
  let params: UrlParams = vec![("token", key)];
  let _ = send("auth.test", &params).await;
}
