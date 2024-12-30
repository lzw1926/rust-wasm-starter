use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
        // 声明 JS 的 Math.random 方法
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen(js_name = getCsrfToken)]
pub fn generate_csrf_token(length: u32) -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let charset_length = charset.len() as f64;
    
    let mut token = String::with_capacity(length as usize);
    
    for _ in 0..length {
        let index = (random() * charset_length) as usize;
        token.push(charset[index] as char);
    }
    
    token
}

#[wasm_bindgen(js_name=validate)]
pub fn validate_token(token: String) -> String {
    token
}