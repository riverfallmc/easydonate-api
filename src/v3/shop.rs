use isahc::{send_async, AsyncReadResponseExt, Request};
use serde::{Deserialize, Serialize};
use crate::{result::{EasyResponse, EasyResult}, util::serde::bool_from_int};
use super::types::user::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct Shop {
  pub id: i64,
  pub rating: Option<f32>,
  pub name: String,
  pub domain: String,
  pub last_domain: String,
  pub delivery_type: String, // "rcon" | ""
  pub description: Option<String>,
  pub user_id: i64,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_active: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_premium: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_copyright: bool,
  pub hide_copyright_till: Option<String>,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_verified: bool,
  pub vk_link: Option<String>,
  pub youtube_link: Option<String>,
  pub discord_link: Option<String>,
  pub twitch_link: Option<String>,
  pub instagram_link: Option<String>,
  pub tiktok_link: Option<String>,
  pub theme_id: i16,
  pub background: String,
  pub logo: String,
  pub favicon: String,
  pub css: Option<String>,

  #[serde(deserialize_with = "bool_from_int")]
  pub enable_background_overlay: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_side_image: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_general_online: bool,
  pub products_image_padding: i16,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_servers: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub test_mode: bool,
  pub test_mode_from: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub side: String,
  pub key: String,
  pub color: String,
  #[serde(deserialize_with = "bool_from_int")]
  pub require_email: bool,
  pub pay_comission: i32,
  pub particles: String,
  pub is_api_moderated: Option<bool>,
  #[serde(deserialize_with = "bool_from_int")]
  pub is_api_pending_moderation: bool,
  pub api_moderated_at: Option<String>,
  pub api_domain: Option<String>,
  pub sort_index: Option<String>,
  #[serde(deserialize_with = "bool_from_int")]
  pub https_redirect: bool,
  #[serde(deserialize_with = "bool_from_int")]
  pub allow_nickname_spaces: bool,
  pub game_id: i16,
  #[serde(deserialize_with = "bool_from_int")]
  pub hide_payment_instruction: bool,
  pub payment_instruction: Option<String>,
  #[serde(deserialize_with = "bool_from_int")]
  pub use_cart: bool,
  pub user: User
}

/// Имплементация
/// https://docs.easydonate.ru/shop/shop
pub async fn get_shop(shop_key: String) -> EasyResult<Shop> {
  let request = Request::get("https://easydonate.ru/api/v3/shop")
    .header("Shop-Key", shop_key)
    .body(())?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<Shop>>(&body)?;

  Ok(des.result()?)
}