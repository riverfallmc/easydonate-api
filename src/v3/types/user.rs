use serde::{Deserialize, Serialize};
use crate::util::serde::bool_from_int;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  pub id: i64,
  #[serde(deserialize_with = "bool_from_int")]
  pub dark_mode: bool,
  pub name: String,
  pub email: String,
  pub avatar_url: String,
  pub permissions: Option<Vec<String>>,
  pub is_activated: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub notify_payments: bool,
  pub activated_at: Option<String>,
  pub last_login: Option<String>,
  pub balance: i64,
  pub cashback: i64,
  #[serde(deserialize_with = "bool_from_int")]
  pub premium: bool,
  pub created_at: String,
  pub updated_at: String,
  pub username: String,
  pub surname: Option<String>,
  pub deleted_at: Option<String>,
  pub last_seen: String,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_guest: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_superuser: bool,
  pub created_ip_address: Option<String>,
  pub last_ip_address: Option<String>,
  #[serde(deserialize_with = "bool_from_int")]
  pub confirm_payouts: bool,
  pub confirmation_code: Option<i32>, // dk
  #[serde(deserialize_with = "bool_from_int")]
  pub confirm_signin: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub payouts_enabled: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub payout_new_schema: bool,
  pub bank_account: Option<String>
}