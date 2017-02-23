# golos-client-rs

Rust клиент для блокчейна Голос. В процессе разработки.

# Пример использования

golos-client-rs пока не опубликован на crates.io, поэтому в Cargo.toml следует использовать git:

```rust
[dependencies]
golos_client = { git = "https://github.com/cyberpunk-ventures/golos-client-rs" }
```

```rust
extern crate golos_client;
use golos::*;

let api = "database_api".to_string();
let api_method = "get_dynamic_global_properties".to_string();
let args = vec![];

let response: Result<serde_json::Value, GolosError> = call(api, api_method, args);
```

# Дорожная карта

* Добавить функции для различных методов API
* Внедрить futures
* Внедрить websockets
* Изучить использование [jsonrpc-core](https://github.com/ethcore/jsonrpc) из Ethereum Parity
* Добавить стракты для ответов API
* Больше тестов и документации
