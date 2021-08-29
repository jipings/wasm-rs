
use wasmer::{imports, wat2wasm, Function, Instance, Module, NativeFunc, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> anyhow::Result<()> {
    // First we create a simple Wasm program to use with Wasmer. 
    // We use the WebAssembly text format and use `wasmer::wat2wasm` to compile
    // it into a WebAssembly binary. 
    //
    // Most WebAssembly programs come from compiling source code in a high level
    // language and will already be in the binary format.
    let wasm_bytes = wat2wasm(
        br#"
(module
  ;; First we define a type with no parameters and no results.
  (type $no_args_no_rets_t (func (param) (result)))
  ;; Then we declare that we want to import a function named "env" "say_hello" with
  ;; that type signature.
  (import "env" "say_hello" (func $say_hello (type $no_args_no_rets_t)))
  ;; Finally we create an entrypoint that calls our imported function.
  (func $run (type $no_args_no_rets_t)
    (call $say_hello))
  ;; And mark it as an exported function named "run".
  (export "run" (func $run)))
"#,
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    let module = Module::new(&store, wasm_bytes)?;

    fn say_hello_world() {
        println!("hello, world!")
    }

    // We then create an import object so that the `Module`'s imports can be satisfied. 
    let import_object = imports! {
        // We use the default namespace "env"
        "env" => {
            // And call our function "say_hello". 
            "say_hello" => Function::new_native(&store, say_hello_world),
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;

    run_func.call()?;

    Ok(())
}  