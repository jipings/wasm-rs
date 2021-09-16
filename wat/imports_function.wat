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