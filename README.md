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

# Note

Currently, these bindings only work on Windows because of some strange errors
arising from bindgen using `u128` on the FFI layer and subsequent warnings from
the compiler because of this. It's likely that there isn't much left to do to
get a Linux & MacOS version of this working but as of right now it proved too
difficult to work out what was wrong on Nix platforms.

## Dependencies

These are required when using `janet-rs-sys` in a project:

* Meson: Builds Janet
* Ninja: Used by Meson
* Clang: Bindgen uses libclang

## Dev Dependencies

### Initialize submodules

```
$ git submodule update --init --recursive
```

### Windows

**Visual Studio Developer Command Prompt is required to build on Windows during
development of `janet-rs-sys`.**
