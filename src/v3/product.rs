use isahc::{send_async, AsyncReadResponseExt, Request};
use crate::result::{EasyResponse, EasyResult};
use super::types::product::Product;

/// Имплементация
/// https://docs.easydonate.ru/shop/product
pub async fn get_product(shop_key: String, id: i64) -> EasyResult<Product> {
  let request = Request::get(&format!("https://easydonate.ru/api/v3/shop/product/${id}"))
    .header("Shop-Key", shop_key)
    .body(())?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<Product>>(&body)?;

  Ok(des.result()?)
}