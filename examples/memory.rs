use std::mem;
use wasmer::{imports, wat2wasm, Bytes, Instance, Module, NativeFunc, Pages, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use std::fs;


fn main() -> anyhow::Result<()> {
    let wasm_str = fs::read_to_string("./wat/memory.wasm")?;
    
    let wasm_bytes = wat2wasm(
        wasm_str.as_bytes(),
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    let module = Module::new(&store, wasm_bytes)?;

    // We then create an import object so that the `Module`'s imports can be satisfied. 
    let import_object = imports! { };

    println!("Instantiating module...");

    let instance = Instance::new(&module, &import_object)?;

    // The module exports some utility functions, let's get them
    //
    // These function will be used later in this example. 
    let mem_size: NativeFunc<(), i32> = instance.exports.get_native_function("mem_size")?;
    let get_at: NativeFunc<i32, i32> = instance.exports.get_native_function("get_at")?;
    let set_at: NativeFunc<(i32, i32), ()> = instance.exports.get_native_function("set_at")?;
    let memory = instance.exports.get_memory("memory")?;

    // We now have an instance ready to be used. 
    // 
    // We will start by querying the most intersting information 
    // about the memory: its size. There are mainly two ways of getting this:
    // * the size as a number of `Page` s
    // * the size as a number of bytes
    // 
    // The size in bytes can be found either by querying its pages or by
    // querying the memory directly. 
    println!("Querying memory size...");
    assert_eq!(memory.size(), Pages::from(1));
    assert_eq!(memory.size().bytes(), Bytes::from(65536 as usize));
    assert_eq!(memory.data_size(), 65536);

    // Sometimes, the guest module may also export a function to let you 
    // query the memory. Here we have a `mem_size` funciton, let's try it:
    let result = mem_size.call()?;
    println!("Memory size: {:?}", result);
    assert_eq!(Pages::from(result as u32), memory.size());

    // Now that we know the size of our memory, it's time to see how we can change this. 
    // A memory can be grown to allow storing more thing into it. 
    // Let's see how we can do that:
    println!("Growing memory...");
    memory.grow(2)?;
    assert_eq!(memory.size(), Pages::from(3));
    assert_eq!(memory.data_size(), 65536 * 3);

    // Now that we know how to query and adjust the size of the memory,
    // let's see how we can write to it or read from it. 
    // 
    // We'll only focus on how to do this using exported functions, the goal
    // is to show how to work with memory addresses. Here we'll use absolute 
    // addresses to write and read a value. 
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
