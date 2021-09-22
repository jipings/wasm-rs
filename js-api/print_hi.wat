(module
    (import "console" "log" (func $log (param i32 i32)))
    (import "js" "mem" (memory 1))
    (data (i32.const 0) "Hi")
    (func (export "writeHi")
        i32.const 0 ;; pass offset 0 to log
        i32.const 2 ;; pass length 2 to log
        call $log
    )
)

;; 我们可以使用数据（data）段把字符串内容写入到一个全局内存中。
;; 数据段允许字符串字节在实例化时被写在一个指定的偏移量。而且，它与原生的可执行格式中的数据(.data) 段是类似的。 
