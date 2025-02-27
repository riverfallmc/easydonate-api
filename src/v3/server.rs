use isahc::{send_async, AsyncReadResponseExt, Request};
use crate::result::{EasyResponse, EasyResult};
use super::types::server::Server;

/// Имплементация
/// https://docs.easydonate.ru/shop/server
pub async fn get_server(shop_key: String, id: i64) -> EasyResult<Server> {
  let request = Request::get(&format!("https://easydonate.ru/api/v3/shop/server/${id}"))
    .header("Shop-Key", shop_key)
    .body(())?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<Server>>(&body)?;

  des.result()
}