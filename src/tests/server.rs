use std::{env, process::ExitCode};
use crate::{result::EasyResult, v3::{server::get_server, types::server::Server}};

/// самое главное
async fn test() -> EasyResult<Server> {
  // ключ магазина
  //
  // для примера берём переменную окружения SHOP_KEY
  // которая и хранит в себе ключ магазина
  let shop_key = env::var("SHOP_KEY")?;

  // отправляем запрос на сервер
  let server = get_server(shop_key, 228)
    .await?;

  // выводим в консоль ответ сервера
  dbg!(&server);

  Ok(server)
}

#[tokio::test]
pub async fn server() -> ExitCode {
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