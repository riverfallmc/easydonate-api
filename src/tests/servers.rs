use std::{env, process::ExitCode};
use crate::{result::EasyResult, v3::{servers::get_servers, types::server::Server}};

/// самое главное
async fn test() -> EasyResult<Vec<Server>> {
  // ключ магазина
  //
  // для примера берём переменную окружения SHOP_KEY
  // которая и хранит в себе ключ магазина
  let shop_key = env::var("SHOP_KEY")?;

  // отправляем запрос на сервер
  let servers = get_servers(shop_key)
    .await?;

  // выводим в консоль ответ сервера
  dbg!(&servers);

  Ok(servers)
}

#[tokio::test]
pub async fn servers() -> ExitCode {
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