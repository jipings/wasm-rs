(module
    (global $some (import "env" "some") f32)
    (global $other (import "env" "other") (mut f32))

    (func (export "get_some") (result f32) (global.get $some))
    (func (export "get_other") (result f32) (global.get $other))

    (func (export "set_other") (param f32) (global.set $other (local.get 0)))
)