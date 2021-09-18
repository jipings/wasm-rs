(module
    ;; All our callbacks will take 2 i32s and return an i32. 
    ;; Wasm tables are not limited to 1 type of function, 
    ;; but the code using the table must have code to handle the type it finds. 
    (type $callback_t (func (param i32 i32) (result i32)))

    ;; We'll call a callback by passing a table index as an i32 and 
    ;; then the two arguments that the function expects. 
    (type $call_callback_t (func (param i32 i32 i32) (result i32)))

    ;; Our table of functions that's exactly size 3 (min 3, max 3). 
    (table $t1 3 6 funcref)

    ;; Call the functions at the given index with the two supplied arguments. 
    (func $call_callback (type $call_callback_t) (param $idx i32)
                                                 (param $arg1 i32) (param $arg2 i32)
                                                 (result i32)    
        (call_indirect (type $callback_t)
                       (local.get $arg1) (local.get $arg2)
                       (local.get $idx)
        )
    )

    ;; A default function that we'll pad the table with. 
    ;; This function doubles both its inputs and then sums them. 
    (func $default_fn (type $callback_t) (param $a i32) (param $b i32) (result i32)
        (i32.add
            (i32.mul (local.get $a) (i32.const 2))
            (i32.mul (local.get $b) (i32.const 2))
        )
    )

    ;; Fill our table with the default function. 
    (elem $t1 (i32.const 0) $default_fn $default_fn $default_fn)

    ;; Export things for the host to call. 
    (export "call_callback" (func $call_callback))
    (export "__indirect_function_table" (table $t1))
)