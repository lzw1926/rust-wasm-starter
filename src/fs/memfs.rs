use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Mfs {
    files: HashMap<String, Vec<u8>>,
}

#[wasm_bindgen(js_class=Mfs)]
impl Mfs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Mfs {
            files: HashMap::new(),
        }
    }

    #[wasm_bindgen(js_name=writeFile)]
    pub fn write_file(&mut self, path: String, content: Vec<u8>) {
        self.files.insert(path, content);
    }

    #[wasm_bindgen(js_name=readFile)]
    pub fn read_file(&self, path: String) -> Option<Vec<u8>> {
        self.files.get(&path).cloned()
    }

    #[wasm_bindgen(js_name=deleteFile)]
    pub fn delete_file(&mut self, path: String) -> bool {
        self.files.remove(&path).is_some()
    }

    #[wasm_bindgen(js_name=exists)]
    pub fn exists(&self, path: String) -> bool {
        self.files.contains_key(&path)
    }

    #[wasm_bindgen(js_name=listFiles)]
    pub fn list_files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    #[wasm_bindgen(js_name=clear)]
    pub fn clear(&mut self) {
        self.files.clear();
    }

    #[wasm_bindgen(js_name=getTotalSize)]
    pub fn get_total_size(&self) -> usize {
        self.files.values().map(|file| file.len()).sum()
    }
}

// impl Default for Mfs {
//     fn default() -> Self {
//         Self::new()
//     }
// }
