//! A Wasm module can export entities, like functions, memories
//! 
//! This example illustrates how to use exported memories. 
//! 
//!  
use wasmer::{imports, wat2wasm, Array, Instance, Module, Store, WasmPtr};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let wasm_bytes = wat2wasm(
    br#"
  (module
      (memory (export "mem") 1)
  
      (global $offset i32 (i32.const 42))
      (global $length (mut i32) (i32.const 13))
  
      (func (export "load") (result i32 i32)
          global.get $offset
          global.get $length)
  
      (data (global.get $offset) "Hello, World!")
  )
    "#
  )?;

  let store = Store::new(&Universal::new(Cranelift::default()).engine());

  println!("Compiling module...");
  let module = Module::new(&store, wasm_bytes)?;

  let import_object = imports! {};
  
  println!("Instantiating module...");

  let instance = Instance::new(&module, &import_object)?;

  let load = instance.exports.get_native_function::<(), (WasmPtr<u8, Array>, i32)>("load")?;

  let memory = instance.exports.get_memory("mem")?;

  println!("Memory size (pages) {:?}", memory.size());
  println!("Memory size (bytes) {:?}", memory.data_size());

  let (ptr, length) = load.call()?;
  println!("String offset: {:?}", ptr.offset());
  println!("String length: {:?}", length);

  // We will get bytes out of the memory so we need to decode them into a string. 
  let str = ptr.get_utf8_string(memory, length as u32).unwrap();
  println!("Memory contents: {:?}", str);

  // What about changing the contents of the memory with a more appropriate string?
  //
  // To do that, we'll dereference our pointer and change the content of each `Cell`
  let new_str = b"Hello, Wasmer!";
  let values = ptr.deref(memory, 0, new_str.len() as u32).unwrap();

  for i in 0..new_str.len() {
    values[i].set(new_str[i]);
  }

  // And now, let's see the result. 
  // 
  // Since the new strings is bigger than older one, we query the length again.
  // The offset remains the same as before. 
  println!("New string length: {:?}", new_str.len());
  
  let str = ptr.get_utf8_string(memory, new_str.len() as u32).unwrap();
  println!("New memory contents: {:?}", str);
  

  Ok(())
}