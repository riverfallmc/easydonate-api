use isahc::{send_async, AsyncReadResponseExt, Request};
use crate::result::{EasyResponse, EasyResult};
use super::types::product::Product;

/// Имплементация
/// https://docs.easydonate.ru/shop/products
pub async fn get_products(shop_key: String) -> EasyResult<Vec<Product>> {
  let request = Request::get("https://easydonate.ru/api/v3/shop/products")
    .header("Shop-Key", shop_key)
    .body(())?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<Vec<Product>>>(&body)?;

  Ok(des.result()?)
}