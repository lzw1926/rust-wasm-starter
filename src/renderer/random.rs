use wasm_bindgen::prelude::wasm_bindgen;
use std::f64::consts::PI;

#[wasm_bindgen(js_name=randomPattern)]
pub fn random_pattern(width: u32, height: u32, delta: u32) -> Vec<u8> {
    let mut pattern = vec![0; (width * height * 4) as usize];
    for i in 0..height {
        for j in 0..width {
            let idx = ((i * width + j) * 4) as usize;
            
            // 计算相对位置 (0.0 到 1.0 之间)
            let x = i as f64 / height as f64;
            let y = j as f64 / width as f64;
            
            // 使用 delta 创建动态偏移
            let time_offset = (delta as f64) * 0.01;
            
            // 计算对角线渐变进度 (0.0 到 1.0)
            let diagonal_progress = (x + y) * 0.5 + time_offset;
            
            // 创建三个错开的渐变相位
            let r = ((diagonal_progress * 2.0 * PI).sin() * 0.5 + 0.5) * 255.0;
            let g = ((diagonal_progress * 2.0 * PI + PI * 0.33).sin() * 0.5 + 0.5) * 255.0;
            let b = ((diagonal_progress * 2.0 * PI + PI * 0.66).sin() * 0.5 + 0.5) * 255.0;
            
            pattern[idx] = r as u8;     // R
            pattern[idx + 1] = g as u8; // G
            pattern[idx + 2] = b as u8; // B
            pattern[idx + 3] = 255;     // A
        }
    }
    pattern
}