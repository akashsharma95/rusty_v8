use rusty_v8 as v8;

#[test]
fn wee8() {
  use v8::wee8::*;
  unsafe {
    let engine = wasm_engine_new();
    let store = wasm_store_new(engine);
    wasm_store_delete(store);
    wasm_engine_delete(engine);

    let params = &mut wasm_valtype_vec_t::default() as *mut wasm_valtype_vec_t;
    wasm_valtype_vec_new_empty(params);

    let results = &mut wasm_valtype_vec_t::default() as *mut wasm_valtype_vec_t;
    wasm_valtype_vec_new_empty(results);

    let hello_type = wasm_functype_new(params, results);
    wasm_functype_delete(hello_type);
  }
}

#[test]
fn wee8_hello() {
  use std::ptr;
  use v8::wee8::*;
  unsafe {
    let engine = wasm_engine_new();
    let store = wasm_store_new(engine);
  
    // Load binary.
    // wasm_byte_vec_t binary;
    let binary = &mut wasm_byte_vec_t::default() as *mut wasm_byte_vec_t;
    wasm_byte_vec_new_uninitialized(binary, 10);
  
    // Compile.
    let module = wasm_module_new(store, binary);
  
    wasm_byte_vec_delete(binary);
  
    // Create external print functions.
    let params = &mut wasm_valtype_vec_t::default() as *mut wasm_valtype_vec_t;
    wasm_valtype_vec_new_empty(params);

    let results = &mut wasm_valtype_vec_t::default() as *mut wasm_valtype_vec_t;
    wasm_valtype_vec_new_empty(results);

    let hello_type = wasm_functype_new(params, results);
    let hello_func = wasm_func_new(store, hello_type, None);
  
    wasm_functype_delete(hello_type);
  
    // Instantiate.
    let externs = wasm_func_as_extern(hello_func);
    let imports = WASM_ARRAY_VEC(externs);
    let instance = wasm_instance_new(store, module, imports, ptr::null_mut());
  
    wasm_func_delete(hello_func);
  
    // Extract export.
    let exports = &mut wasm_extern_vec_t::default() as *mut wasm_extern_vec_t;
    wasm_instance_exports(instance, exports);
    if (*exports).size == 0 {
      println!("> Error accessing exports!\n");
    }
    
    let run_func = wasm_extern_as_func((*exports).data[0]);
    if run_func == ptr::null_mut() {
      println!("> Error accessing export!\n");
    }
  
    wasm_module_delete(module);
    wasm_instance_delete(instance);
  
    // Call.
    let args = wasm_val_t::default();
    let results = &mut wasm_val_t::default() as *mut wasm_val_t;

    if wasm_func_call(run_func, &args, results) == ptr::null_mut() {
      println!("> Error calling function!\n");
    }
  
    wasm_extern_vec_delete(exports);
  
    // Shut down.
    wasm_store_delete(store);
    wasm_engine_delete(engine);
  }
}