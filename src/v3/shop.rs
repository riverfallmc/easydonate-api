use isahc::{send_async, AsyncReadResponseExt, Request};
use crate::result::{EasyResponse, EasyResult};

use super::types::shop::Shop;

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

  des.result()
}