# Rust EasyDonate API
![version](https://img.shields.io/crates/v/easydonate-api
)

Неофициальная имплементация [API EasyDonate](https://docs.easydonate.ru/) в виде крейта на языке Rust.

# В краце
* [Установка](#установка)
* [Использование (описание API)](#использование)
  * [Информация о магазине](#get_shop)
  * [Список товаров](#get_products)
  * [Информация о товаре](#get_product)
  * [Список серверов](#get_servers)
  * [Информация о сервере](#get_server)

# Установка
```bash
cargo add easydonate-api
```

# Использование
## Работа с магазином
### ``get_shop()``
``GET`` https://easydonate.ru/api/v3/shop\
``?`` Возвращает объект магазина \
``(...)`` ``shop_key: String``

```rs
use easydonate_api::v3::shop::get_shop;

#[tokio::main]
async fn main() {
  let shop = get_shop(std::env::var("SHOP_KEY"))
    .await
    .unwrap();

  println!("{shop:?}");
}
```

[Более подробный пример в виде теста.](./src/tests/shop.rs)

### ``get_products()``
``GET`` https://easydonate.ru/api/v3/shop/products\
``?`` Возвращает список (массив) продуктов \
``(...)`` ``shop_key: String``

```rs
use easydonate_api::v3::products::get_products;

#[tokio::main]
async fn main() {
  let products = get_products(std::env::var("SHOP_KEY"))
    .await
    .unwrap();

  println!("{products:?}");
}
```

[Более подробный пример в виде теста.](./src/tests/products.rs)

### ``get_product()``
``GET`` https://easydonate.ru/api/v3/shop/product/{id} \
``?`` Возвращает продукт \
``(...)`` ``shop_key: String``, ``id: i64``

```rs
use easydonate_api::v3::product::get_products;

#[tokio::main]
async fn main() {
  let product = get_product(std::env::var("SHOP_KEY"), 10) // 10 - айди продукта
    .await
    .unwrap();

  println!("{product:?}");
}
```

[Более подробный пример в виде теста.](./src/tests/product.rs)

### ``get_servers()``
``GET`` https://easydonate.ru/api/v3/servers \
``?`` Возвращает список (массив) серверов \
``(...)`` ``shop_key: String``

```rs
use easydonate_api::v3::servers::get_servers;

#[tokio::main]
async fn main() {
  let servers = get_servers(std::env::var("SHOP_KEY"))
    .await
    .unwrap();

  println!("{servers:?}");
}
```

[Более подробный пример в виде теста.](./src/tests/servers.rs)

### ``get_server()``
``GET`` https://easydonate.ru/api/v3/server/{id} \
``?`` Возвращает сервер \
``(...)`` ``shop_key: String``, ``id: i64``

```rs
use easydonate_api::v3::server::get_server;

#[tokio::main]
async fn main() {
  let server = get_server(std::env::var("SHOP_KEY"), 10) // 10 - айди сервера
    .await
    .unwrap();

  println!("{server:?}");
}
```

[Более подробный пример в виде теста.](./src/tests/server.rs)