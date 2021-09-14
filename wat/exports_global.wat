(module
    (global $one (export "one") f32 (f32.const 1))
    (global $some (export "some") (mut f32) (f32.const 0))

    (func (export "get_one") (result f32) (global.get $one))
    (func (export "get_some") (result f32) (global.get $some))

    (func (export "set_some") (param f32) (global.set $some ( local.get 0 )))
)