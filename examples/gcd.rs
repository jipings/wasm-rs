
use wasmer::{imports, wat2wasm, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use std::fs;

fn main() -> anyhow::Result<()> {
    let gcd_str = fs::read_to_string("./wat/gcd.wasm")?;
    
    // First we create a simple Wasm program to use with Wasmer. 
    // We use the WebAssembly text format and use `wasmer::wat2wasm` to compile
    // it into a WebAssembly binary. 
    //
    // Most WebAssembly programs come from compiling source code in a high level
    // language and will already be in the binary format.
    
    let wasm_bytes = wat2wasm(
        gcd_str.as_bytes(),
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    let module = Module::new(&store, wasm_bytes)?;

    // We then create an import object so that the `Module`'s imports can be satisfied. 
    let import_object = imports! { };

    let instance = Instance::new(&module, &import_object)?;

    let run_func = instance.exports.get_function("gcd")?.native::<(i32, i32), i32>()?;

    let result = run_func.call(4, 5)?;

    println!("Results of `gcd`: {:?}", result);

    Ok(())
}  