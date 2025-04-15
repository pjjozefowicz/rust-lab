use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

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
                Ok(content) => match extract_methods(&content) {
                    Ok(methods) => {
                        println!("Available RPC methods:");
                        for method in methods {
                            println!("- {}", method);
                        }
                    }
                    Err(err) => eprintln!("Failed to extract methods: {}", err),
                },
                Err(err) => eprintln!("Failed to read response body: {}", err),
            }
        }
        Err(err) => eprintln!("Request failed: {}", err),
    }
}

fn extract_methods(json_str: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let parsed: Value = serde_json::from_str(json_str)?;
    let methods_array = parsed["result"]["methods"]
        .as_array()
        .ok_or("Expected an array of methods")?;

    let mut methods = Vec::new();
    for val in methods_array {
        if let Some(s) = val.as_str() {
            methods.push(s.to_string());
        }
    }

    Ok(methods)
}
