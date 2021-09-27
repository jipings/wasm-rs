
use anyhow::bail;
use std::fmt;
use wasmer::{imports, wat2wasm, Function, Instance, Module, NativeFunc, RuntimeError, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use std::fs;

// First we need to create an error type that we'll use to signal the end of execution. 
#[derive(Debug, Clone, Copy)]
struct ExitCode(u32);

// This type must implement `std::error::Error` so we must also implement `std::fmt::Display` for it. 
impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// And then we implement `std::error::Error`
impl std::error::Error for ExitCode {}

fn main() -> anyhow::Result<()> {
    let wasm_str = fs::read_to_string("./wat/interrupt.wasm")?;
    
    let wasm_bytes = wat2wasm(
        wasm_str.as_bytes(),
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    let module = Module::new(&store, wasm_bytes)?;

    // We declare the host function that we'll use to terminate execution. 
    fn early_exit() {
        // This is where it happens. 
        RuntimeError::raise(Box::new(ExitCode(1)));
    }

    // We then create an import object so that the `Module`'s imports can be satisfied. 
    let import_object = imports! { 
        "env" => {
            "early_exit" => Function::new_native(&store, early_exit),
        }
    };

    println!("Instantiating module...");

    let instance = Instance::new(&module, &import_object)?;

    println!("Calling `run` function...");
    let run_func: NativeFunc<(i32, i32), i32> = instance.exports.get_native_function("run")?;

    // When we call a function it can either succeed or fail. We expect it to fail. 
    match run_func.call(1, 7) {
        Ok(result) => {
            bail!(
                "Expected early termination with `ExitCode`, found: {}",
                result
            );
        }
        // In case of a failure, which we expect, we attempt to downcast the error into the error
        // type that we were expecting. 
        Err(e) => match e.downcast::<ExitCode>() {
            // We found the exit code used to terminate execution. 
            Ok(exit_code) => {
                println!("Exited early with exit code: {}", exit_code);
                Ok(())
            }
            Err(e) => {
                bail!("Unknown error `{}` found. expected `ErrorCode`", e);
            }
        },
    }
}  