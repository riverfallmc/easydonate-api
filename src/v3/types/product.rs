use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
  pub id: i64,
  pub name: String,
  pub price: u64,
  pub old_price: Option<u64>,
  pub r#type: String,
  pub number: u64,
  pub commands: Vec<String>,
  pub withdraw_commands: Option<Vec<String>>,
  pub description: Option<String>,
  pub image: String,
  //pub first_delete: dk what it is,
  pub shop_id: i64,
  pub created_at: String,
  pub updated_at: String,
  pub sort_index: u16,
}