use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RpcResponse {
    jsonrpc: String,
    result: RpcResult,
    id: u32,
}

#[derive(Debug, Deserialize)]
struct RpcResult {
    methods: Vec<String>,
}

fn main() {
    let body = r#"
    {
        "id": 1,
        "jsonrpc": "2.0",
        "method": "rpc_methods",
        "params": []
    }
    "#;

    let client = Client::new();

    let response = client
        .post("http://localhost:9944")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();

    match response {
        Ok(resp) => {
            let text = resp.text();
            match text {
                Ok(content) => {
                    let rpc: RpcResponse =
                        serde_json::from_str(&content).expect("Deserialization failed");

                    println!("Available RPC methods:");
                    for method in rpc.result.methods {
                        println!("- {}", method);
                    }
                }
                Err(err) => eprintln!("Failed to read response body: {}", err),
            }
        }
        Err(err) => eprintln!("Request failed: {}", err),
    }
}
