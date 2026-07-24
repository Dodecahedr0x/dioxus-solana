#![cfg(target_arch = "wasm32")]
use dioxus_solana_web::js::{get, get_string};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn reads_nested_properties() {
    // Build { name: "Phantom", nested: { x: 5 } } via JS.
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"name".into(), &"Phantom".into()).unwrap();
    let nested = js_sys::Object::new();
    js_sys::Reflect::set(&nested, &"x".into(), &JsValue::from_f64(5.0)).unwrap();
    js_sys::Reflect::set(&obj, &"nested".into(), &nested).unwrap();

    assert_eq!(get_string(&obj, "name").as_deref(), Some("Phantom"));
    let n = get(&obj, "nested").unwrap();
    assert_eq!(get(&n, "x").unwrap().as_f64(), Some(5.0));
    assert!(get(&obj, "missing").is_none());
}
