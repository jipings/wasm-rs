//! 1. How to load a Wasm modules as bytes
//! 2. How to compile the module
//! 3. How to create an instance of the module

use wasmer::{imports, wat2wasm, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let wasm_bytes = wat2wasm(
        br#"
(module
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#,
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    println!("Compiling module...");

    let module = Module::new(&store, wasm_bytes)?;

    let imports_object = imports! {};
    println!("Instantiating module...");

    let instance = Instance::new(&module, &imports_object)?;

    let add_one = instance
        .exports
        .get_function("add_one")?
        .native::<i32, i32>()?;
    println!("Calling `add_one` function...");

    let result = add_one.call(1)?;

    println!("Results of `add_one`: {:?}", result);

    assert_eq!(result, 2);

    Ok(())
}

#[test]
fn test_exported_function() -> Result<(), Box<dyn std::error::Error>> {
    main()
}