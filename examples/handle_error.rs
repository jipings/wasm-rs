//! In this example we'll see how to handle such errors in the most
//! basic way. To do that we'll use a Wasm module that we know will
//! produce an error.
//! 
use wasmer::{imports, wat2wasm, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let wasm_bytes = wat2wasm(
    br#"
  (module
  (type $do_div_by_zero_t (func (result i32)))
  (func $do_div_by_zero_f (type $do_div_by_zero_t) (result i32)
  i32.const 4
  i32.const 0
  i32.div_s)
  (type $div_by_zero_t (func (result i32)))
  (func $div_by_zero_f (type $div_by_zero_t) (result i32)
  call $do_div_by_zero_f)
  (export "div_by_zero" (func $div_by_zero_f)))
  "#,
  )?;

  let store = Store::new(&Universal::new(Cranelift::default()).engine());
  println!("Compiling module...");

  // Let's compile the Wasm module.
  let module = Module::new(&store, wasm_bytes)?;

  let import_object = imports! {};

  println!("Instantiating module...");

  let instance = Instance::new(&module, &import_object)?;

  let div_by_zero = instance
    .exports
    .get_function("div_by_zero")?
    .native::<(), i32>()?;
  println!("Calling `div_by_zero` function...");

  let result = div_by_zero.call();

  match result {
    Ok(_) => {
      panic!("div_by_zero did not error");
    }
    Err(e) => {
      println!("Error caught from `div_by_zero`: {}", e.message());

      let frames = e.trace();
      let frames_len = frames.len();
      println!("{}", frames_len);
      for i in 0..frames_len {
        println!(
          "  Frame #{}: {:?}::{:?}",
          frames_len - i,
          frames[i].module_name(),
          frames[i].function_name().or(Some("<func>")).unwrap()
        );
      }
    }
  }


 Ok(())
}