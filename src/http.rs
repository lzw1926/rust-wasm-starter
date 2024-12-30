use wasm_bindgen::prelude::*;
use reqwest;
use web_sys::{console, window};


fn guarantee_http_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    let current_host = window().unwrap().origin();
    format!("{}{}", current_host, url)
}

#[wasm_bindgen(js_name=get)]
pub async fn fetch_with_reqwest(mut url: String) -> Result<String, JsValue> {
    url = guarantee_http_url(&url);
    
    let response = reqwest::Client::new().get(&url)
        .send()
        .await
        .map_err(|e| {
            console::log_1(&JsValue::from_str(&e.to_string()));
            JsError::from(&e)
        })?;

        // .map_err(|e| JsError::from(&e))?;
        
    let text = response.text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(text)
}
// POST 请求示例
#[wasm_bindgen(js_name=post)]
pub async fn post_with_reqwest(url: String, body: String) -> Result<String, JsValue> {
    let url = guarantee_http_url(&url);
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| JsError::from(&e))?;
        
    let text = response.text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
    Ok(text)
}

#[wasm_bindgen(js_name=getGithub)]
pub async fn run() -> Result<JsValue, JsValue> {
    let res = reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/main")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| JsError::from(&e))?;
    
    let text = res.text().await.map_err(|e| JsError::from(&e))?;
    // let branch_info: Branch = serde_json::from_str(&text).unwrap();
    Ok(JsValue::from_str(&text))
}