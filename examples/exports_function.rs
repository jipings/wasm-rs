//! A Wasm module can export entities, like functions, memories, globals and tables. 
//! 
//! 1. Dynamic functions, where parameters and results are of a slice of `Value`;
//! 2. Native function, where parameters and results are statically typed Rust values. 
use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let wasm_bytes = wat2wasm(
    r#"(module
      (type $sum_t (func (param i32 i32) (result i32)))
      (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
          local.get $x 
          local.get $y
          i32.add)
      (export "sum" (func $sum_f))
  )"#
  .as_bytes(),
  )?;

  let store = Store::new(&Universal::new(Cranelift::default()).engine());

  println!("Comilping module...");
  let module = Module::new(&store, wasm_bytes)?;

  let import_object = imports! {};

  println!("Instantiating module...");
  let instance = Instance::new(&module, &import_object)?;

  let sum = instance.exports.get_function("sum")?;

  println!("Calling `sum` function...");

  let args = [Value::I32(1), Value::I32(2)];
  let result = sum.call(&args)?;

  println!("Results: {:?}", result);
  assert_eq!(result.to_vec(), vec![Value::I32(3)]);

  let sum_native = sum.native::<(i32, i32), i32>()?;
  println!("Calling `sum` function (natively)...");
  let result = sum_native.call(3,4)?;

  println!("Results: {:?}", result);
  assert_eq!(result, 7);

  Ok(())
}