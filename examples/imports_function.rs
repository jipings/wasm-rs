//! A Wasm module can import entities, like functions, memories, globals and tables. 
//! 
//! 1. Dynamic functions, where parameters and results are of a slice of `Value`;
//! 2. Native function, where parameters and results are statically 
use wasmer::{imports, wat2wasm, Function, FunctionType, Instance, Module, Store, Type, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let wasm_bytes = wat2wasm(
    br#"
    (module
      (func $multiply_dynamic (import "env" "multiply_dynamic") (param i32) (result i32))
      (func $multiply_native (import "env" "multiply_native") (param i32) (result i32))
  
      (type $sum_t (func (param i32) (param i32) (result i32)))
      (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
          (call $multiply_dynamic (local.get $x))
          (call $multiply_native (local.get $y))
          i32.add)
  
      (export "sum" (func $sum_f))
    )
    "#,
  )?;

  let store = Store::new(&Universal::new(Cranelift::default()).engine());

  println!("Compiling module...");

  let module = Module::new(&store, wasm_bytes)?;

  // Create the functions
  let multiply_dynamic_signature = FunctionType::new(vec![Type::I32], vec![Type::I32]);
  let multiply_dynamic = Function::new(&store, &multiply_dynamic_signature, |args| {
    println!("Calling `multiply_dynamic` ...");

    let result = args[0].unwrap_i32() * 2;
    println!("Result of `multiply_dynamic`: {:?}", result);
    Ok(vec![Value::I32(result)])
  });

  fn multiply(a: i32) -> i32 {
    println!("Calling `multiply_native` ...");
    let result = a * 3;

    println!("Result of `multiply_native`: {:?}", result);
    result
  }

  let multiply_native = Function::new_native(&store, multiply);

  // Create an import object. 
  let import_object = imports! {
    "env" => {
      "multiply_dynamic" => multiply_dynamic,
      "multiply_native" => multiply_native,
    }
  };

  println!("Instantiating module ...");
  // Let's instantiate the Wasm module. 
  let instance = Instance::new(&module, &import_object)?;
  let sum = instance.exports.get_function("sum")?.native::<(i32, i32), i32>()?;

  println!("Calling `sum` function...");

  let result = sum.call(1,2)?;

  println!("Results of `sum`: {:?}", result);
  assert_eq!(result, 8);

  Ok(())
}