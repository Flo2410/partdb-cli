use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
  pub name: String,

  #[serde(rename = "addedDate")]
  pub added_date: String,

  #[serde(rename = "lastModified")]
  pub last_modified: String,

  pub id: u32,
  pub needs_review: bool,
  pub tags: String,
  pub description: String,
  pub comment: String,
  pub favorite: bool,
  pub minamount: f32,
  pub manufacturer_product_url: String,
  pub manufacturer_product_number: String,
  pub total_instock: f32,

  #[serde(rename = "projectBuildPart")]
  pub project_build_part: bool,
}
