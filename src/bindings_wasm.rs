use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = preprocessImage)]
pub fn preprocess_image_wasm(
    data: &[u8],
    window_size: Option<u32>,
    k: Option<f64>,
    target_width: Option<u32>,
) -> Result<Vec<u8>, JsValue> {
    let ws = window_size.unwrap_or(25);
    let kp = k.unwrap_or(0.2) as f32;
    match crate::preprocess_image_sync(data.to_vec(), ws, kp, target_width) {
        Ok(buf) => Ok(buf),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}
