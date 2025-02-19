use std::{env, process::ExitCode};
use crate::{result::EasyResult, v3::shop::{get_shop, Shop}};

/// самое главное
async fn test() -> EasyResult<Shop> {
  // ключ магазина
  //
  // для примера берём переменную окружения SHOP_KEY
  // которая и хранит в себе ключ магазина
  let shop_key = env::var("SHOP_KEY")?;

  // отправляем запрос на сервер
  let shop = get_shop(shop_key)
    .await?;

  // выводим в консоль ответ сервера
  dbg!(&shop);

  Ok(shop)
}

/// на это можно не обращать внимание
///
/// у кого-то просто руки кривые чтобы имплементить трейт Termination ахха
#[tokio::test]
pub async fn shop() -> ExitCode {
  match test().await {
    Ok(res) => {
      println!("{res:?}");
      ExitCode::SUCCESS
    },
    Err(err) => {
      println!("{err:?}");
      ExitCode::FAILURE
    }
  }
}