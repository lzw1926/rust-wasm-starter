use wasm_bindgen::prelude::*;
use reqwest;
use web_sys::{console, window};
use serde_json::{self, Value};
use serde_wasm_bindgen::{self};

fn guarantee_http_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    let current_host = window().unwrap().origin();
    format!("{}{}", current_host, url)
}

fn serialize_resp(value: &String) -> Result<JsValue, JsError> {
    // let serializer: Serializer = Serializer::new().serialize_maps_as_objects(true);
    let data: Value = serde_json::from_str(value).map_err(|e| JsError::new(&format!("JSON parse error: {}", e)))?;

    Ok(serde_wasm_bindgen::to_value(&data)?)
}

#[wasm_bindgen(js_name=get)]
pub async fn fetch_with_reqwest(mut url: String) -> Result<JsValue, JsError> {
    url = guarantee_http_url(&url);
    
    let response = reqwest::Client::new().get(&url)
        .send()
        .await
        .map_err(|e| {
            console::log_1(&JsValue::from_str(&e.to_string()));
            JsError::from(&e)
        })?;
        
    let text = response.text()
        .await?;

    serialize_resp(&text)
}
// POST 请求示例
#[wasm_bindgen(js_name=post)]
pub async fn post_with_reqwest(url: String, body: String) -> Result<JsValue, JsError> {
    let url = guarantee_http_url(&url);
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
        
    let text = response.text()
        .await?;

    serialize_resp(&text)
}
