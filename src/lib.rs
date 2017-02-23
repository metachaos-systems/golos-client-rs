extern crate hyper;

#[macro_use]
extern crate serde_json;
extern crate env_logger;

use hyper::Client;
use std::io::Read;

#[derive(Debug)]
pub enum GolosError {
    CallFailed,
    Http(hyper::Error),
    SerdeJsonParsing(serde_json::Error),
    ReadResponse(std::io::Error),
}

pub enum GolosApi {
    DatabaseApi,
    FollowsApi,
}

pub fn call(api: GolosApi,
            api_method: String,
            args: Vec<String>)
            -> Result<serde_json::Value, GolosError> {
    const RPC_ENDPOINT: &'static str = "http://node.golos.ws/rpc";

    let api_str: String = match api {
        GolosApi::DatabaseApi => "database_api".to_string(),
        GolosApi::FollowsApi => "follow_api".to_string(),
    };

    let value = json!({
        "jsonrpc": "2.0",
        "method": "call",
        "id": "1",
        "params": [api_str, api_method, args]
    });

    let client = Client::new();

    let mut res = try!(client.post(RPC_ENDPOINT)
        .body(&serde_json::to_string(&value).unwrap())
        .send()
        .map_err(GolosError::Http));

    let mut s = String::new();
    try!(res.read_to_string(&mut s).map_err(GolosError::ReadResponse));
    let json: serde_json::Value =
        try!(serde_json::from_str(&s).map_err(GolosError::SerdeJsonParsing));

    match json["error"].is_string() {
        false => Ok(json),
        true => Err(GolosError::CallFailed),
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    #[test]
    fn get_dynamic_props_rpc_call_succeeds() {
        let api = GolosApi::DatabaseApi;
        let api_method = "get_dynamic_global_properties".to_string();
        let args = vec![];
        let response_map = json!(call(api, api_method, args).unwrap());
        assert!(response_map["result"]["head_block_number"].as_u64().unwrap() > 3000000);
    }

    #[test]
    fn get_content_rpc_call_succeeds() {
        let api = GolosApi::DatabaseApi;
        let api_method = "get_content".to_string();
        let args = vec!["hipster".to_string(), "iniciativa-kiber-fonda-po-podderzhke-otkrytogo-iskhodnogo-koda-v-golose".to_string()];
        let response_map = json!(call(api, api_method, args).unwrap());
        assert!(response_map["result"]["title"].as_str().unwrap() == "Инициатива кибер•Фонда по поддержке открытого исходного кода в Голосе");
    }


    #[test]
    fn get_followers_rpc_call_succeeds() {
        let api = GolosApi::FollowsApi;
        let api_method = "get_followers".to_string();
        let args =
            vec!["ontofractal".to_string(), "".to_string(), "blog".to_string(), "100".to_string() ];
        let response_map = json!(call(api, api_method, args).unwrap());
        assert!( !response_map["result"][0]["follower"].as_str().unwrap().is_empty() );
    }
}
