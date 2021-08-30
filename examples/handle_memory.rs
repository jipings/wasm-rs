//! This example illustrates the basics of interacting with Wasm module memory.:
use std::mem;
use wasmer::{imports, wat2wasm, Bytes, Instance, Module, NativeFunc, Pages, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> anyhow::Result<()> {
  let wasm_bytes = wat2wasm(
    r#"
      (module
      (type $mem_size_t (func (result i32)))
      (type $get_at_t (func (param i32) (result i32)))
      (type $set_at_t (func (param i32) (param i32)))
      (memory $mem 1)
      (func $get_at (type $get_at_t) (param $idx i32) (result i32)
      (i32.load (local.get $idx)))
      (func $set_at (type $set_at_t) (param $idx i32) (param $val i32)
      (i32.store (local.get $idx) (local.get $val)))
      (func $mem_size (type $mem_size_t) (result i32)
      (memory.size))
      (export "get_at" (func $get_at))
      (export "set_at" (func $set_at))
      (export "mem_size" (func $mem_size))
      (export "memory" (memory $mem)))
    "#.as_bytes(),
  )?;

  let store = Store::new(&Universal::new(Cranelift::default()).engine());
  println!("Compiling module...");

  let module = Module::new(&store, wasm_bytes)?;

  let import_object = imports! {};
  println!("Instantiating module...");

  let instance = Instance::new(&module, &import_object)?;

  let mem_size: NativeFunc<(), i32> = instance.exports.get_native_function("mem_size")?;
  let get_at: NativeFunc<i32, i32> = instance.exports.get_native_function("get_at")?;
  let set_at: NativeFunc<(i32, i32), ()> = instance.exports.get_native_function("set_at")?;
  let memory = instance.exports.get_memory("memory")?;

  println!("Querying memory size ...");
  assert_eq!(memory.size(), Pages::from(1));
  assert_eq!(memory.size().bytes(), Bytes::from(65536 as usize));
  assert_eq!(memory.data_size(), 65536);

  let result = mem_size.call()?;
  println!("Memory size: {:?}", result);
  assert_eq!(Pages::from(result as u32), memory.size());

  println!("Growing memory...");
  // Here we are requesting 2 more pages for our memory. 
  memory.grow(2)?;
  assert_eq!(memory.size(), Pages::from(3));
  assert_eq!(memory.data_size(), 65536 * 3);

  // Now that we know how to query and adjust the size of the memory,
  // let's see how we can write to it or read from it. 
  //
  // We'll only focus on how to do this using exported functions, the goal is 
  // to show how to work with memory addressses. Here we'll use absolute addresses 
  // to write and read a value. 
  let mem_addr = 0x2220;
  let val = 0xFEFEFFE;

  set_at.call(mem_addr, val)?;

  let result = get_at.call(mem_addr)?;
  println!("Value at {:#x?}: {:?}", mem_addr, result);
  assert_eq!(result, val);

  // Now instead of using hard coded memory addresses, let's try to write
  // something at the end of the second memory page and read it.
  let page_size = 0x1_0000;
  let mem_addr = (page_size * 2) - mem::size_of_val(&val) as i32;
  let val = 0xFEA09;
  set_at.call(mem_addr, val)?;

  let result = get_at.call(mem_addr)?;
  println!("Value at {:#x?}: {:?}", mem_addr, result);
  assert_eq!(result, val);


  Ok(())
}