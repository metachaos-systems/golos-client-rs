# golos-client-rs

Rust http клиент для блокчейна Голос.

# Пример использования

golos-client-rs пока не опубликован на crates.io, поэтому в Cargo.toml следует использовать git:

```rust
[dependencies]
golos_client = { git = "https://github.com/cyberpunk-ventures/golos-client-rs" }
```

Основная функция модуля `call` принимает следующие аргументы: значение enum, например, GolosApi::DatabaseApi, метод API и вектор параметров для данного метода и вовзращает `serde_json::Value`.

```rust
extern crate golos_client;
use golos::*;

let api = GolosApi::DatabaseApi;
let api_method = "get_dynamic_global_properties".to_string();
let args = vec![];

let response: Result<serde_json::Value, GolosError> = call(api, api_method, args);
response["result"]["head_block_number"].as_u64().unwrap() > 3000000; // true
```

# Дорожная карта

* Добавить функции для различных методов API
* Внедрить futures
* Внедрить websockets
* Изучить использование [jsonrpc-core](https://github.com/ethcore/jsonrpc) из Ethereum Parity
* Добавить стракты для ответов API
* Больше тестов и документации
