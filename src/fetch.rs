use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn execute(cmd: String) {
    wasm_bindgen_futures::spawn_local(async move {
        let invoke_result = invoke(cmd.as_str(), JsValue::from_str(""))
            .await
            .as_string()
            .unwrap_or_default();
        web_sys::console::log_1(&format!("invoke_result: {:?}", &invoke_result).into());
    });
}
