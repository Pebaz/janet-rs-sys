# janet-rs-sys

Raw Rust bindings to the Janet Programming Language

```rust
use janet_rs_sys::*;
use std::ffi::*;

fn main()
{
    let result = unsafe { janet_init() };
    assert_eq!(result, 0);

    let env = unsafe { janet_core_env(std::ptr::null_mut()) };

    let code =
        CString::new(r#"(print "Hello World!")"#).unwrap().as_c_str().as_ptr();

    let file = CString::new("hello-world.janet").unwrap().as_c_str().as_ptr();

    let result =
        unsafe { janet_dostring(env, code, file, std::ptr::null_mut()) };
    assert_eq!(result, 0);
}
```
