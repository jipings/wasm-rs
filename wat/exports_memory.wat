(module
    (memory (export "mem") 1)

    (global $offset i32 (i32.const 42))
    (global $length (mut i32) (i32.const 13))

    (func (export "load") (result i32 i32)
        global.get $offset
        global.get $length)

    (data (global.get $offset) "Hello, World!")
)