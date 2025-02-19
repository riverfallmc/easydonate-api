# Rust EasyDonate API
![version](https://img.shields.io/crates/v/easydonate-api
)

Неофициальная имплементация [API EasyDonate](https://docs.easydonate.ru/) в виде крейта на языке Rust.

# Установка
```bash
cargo add easydonate-api
```

# Использование
## Работа с магазином
### ``get_shop()``
``GET`` https://easydonate.ru/api/v3/shop

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