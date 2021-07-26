use rusty_v8 as v8;

#[test]
fn wee8() {
  use v8::wee8::*;
  unsafe {
    let engine = wasm_engine_new();
    let store = wasm_store_new(engine);
    wasm_store_delete(store);
    wasm_engine_delete(engine);

    // let params : *mut wasm_valtype_vec_t;
    // wasm_valtype_vec_new_empty(params);
    // let results : *mut wasm_valtype_vec_t;
    // wasm_valtype_vec_new_empty(results);

    // let hello_type = wasm_functype_new(params, results);
    // wasm_functype_delete(hello_type);
  }
}