(module
    (type $do_div_by_zero_t (func (result i32)))
    (func $do_div_by_zero_f (type $do_div_by_zero_t) (result i32)
      i32.const 4
      i32.const 0
      i32.div_s)

    (type $do_div_by_zero_t (func (result i32)))
    (func $do_div_by_zero_f (type $do_div_by_zero_t) (result i32)
        call $do_div_by_zero_f)
    
    (export "div_by_zero" (func $do_div_by_zero_f))
)