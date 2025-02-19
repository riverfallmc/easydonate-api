use std::{env, process::ExitCode};
use crate::{result::EasyResult, v3::{product::get_product, types::product::Product}};

/// самое главное
async fn test() -> EasyResult<Product> {
  // ключ магазина
  //
  // для примера берём переменную окружения SHOP_KEY
  // которая и хранит в себе ключ магазина
  let shop_key = env::var("SHOP_KEY")?;

  // отправляем запрос на сервер
  let products = get_product(shop_key, 228)
    .await?;

  // выводим в консоль ответ сервера
  dbg!(&products);

  Ok(products)
}

#[tokio::test]
pub async fn product() -> ExitCode {
  match test().await {
    Ok(_) => {
      ExitCode::SUCCESS
    },
    Err(err) => {
      println!("{err:?}");
      ExitCode::FAILURE
    }
  }
}