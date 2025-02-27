use std::collections::HashMap;
use isahc::{send_async, AsyncReadResponseExt, Request};
use serde::{Deserialize, Serialize};
use crate::result::{EasyResponse, EasyResult};

#[derive(Serialize, Debug, Clone)]
pub struct PaymentBody {
  pub customer: String,
  pub server_id: i32,
  pub products: HashMap<String, i32>,
  pub email: String,
  pub coupon: Option<String>,
  pub success_url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct PaymentResponse {
  pub url: String,
  // payment: {...}
}

/// **Неполная** Имплементация
/// https://docs.easydonate.ru/shop/payment-create
pub async fn create_payment(
  shop_key: String,
  payload: PaymentBody
) -> EasyResult<String> {
  let request = Request::get("https://easydonate.ru/api/v3/shop/payment/create")
    .header("Shop-Key", shop_key)
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&payload)?)?;

  let mut response = send_async(request)
    .await?;

  let body = response.text()
    .await?;

  let des = serde_json::from_str::<EasyResponse<PaymentResponse>>(&body)?;

  Ok(des.result()?.url)
}