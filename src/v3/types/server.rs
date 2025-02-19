use serde::{Deserialize, Serialize};
use crate::util::serde::bool_from_int;

use super::product::Product;

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
  pub id: i64,
  pub name: String,
  pub ip: String,
  pub port: u32,
  pub version: String,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_port_hidden: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_ip: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_hidden: bool,
  pub shop_id: i64,
  pub created_at: String,
  pub updated_at: String,
  pub products: Option<Vec<Product>>
}