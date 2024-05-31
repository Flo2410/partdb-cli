use anyhow::Ok;
use reqwest::{
  header::{HeaderMap, HeaderValue},
  Client,
};
use url::Url;

use super::Part;

#[derive(Debug)]
pub struct PartDB {
  pub url: Url,
  pub token: String,

  client: Client,
}

impl PartDB {
  pub fn new(url: Url, token: String) -> anyhow::Result<Self> {
    let mut headers = HeaderMap::new();
    headers.insert(
      "Authorization",
      HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder().default_headers(headers).build()?;
    Ok(Self { url, token, client })
  }

  pub async fn get_parts(&self) -> anyhow::Result<Vec<Part>> {
    let res = self.client.get(self.url.join("/api/parts")?).send().await?;
    Ok(serde_json::from_str::<Vec<Part>>(res.text().await?.as_str())?)
  }

  pub async fn get_part(&self, id: u32) -> anyhow::Result<Part> {
    let res = self
      .client
      .get(self.url.join(format!("/api/parts/{}", id).as_str())?)
      .send()
      .await?;
    Ok(serde_json::from_str(res.text().await?.as_str())?)
  }
}
