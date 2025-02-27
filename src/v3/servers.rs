use isahc::{send_async, AsyncReadResponseExt, Request};
use crate::result::{EasyResponse, EasyResult};

use super::types::server::Server;

/// Имплементация
/// https://docs.easydonate.ru/shop/servers
pub async fn get_servers(shop_key: String) -> EasyResult<Vec<Server>> {
  let request = Request::get("https://easydonate.ru/api/v3/shop/servers")
    .header("Shop-Key", shop_key)
    .body(())?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<Vec<Server>>>(&body)?;

  des.result()
}