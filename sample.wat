(module
  (type (func (param i32)))
  (import "console" "log" (func $log (type 0)))
  (func $test (param i32) (param i32) (param i32)
    (call $log (i32.const 0))
  )
)