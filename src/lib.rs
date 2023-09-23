#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::*;

    #[test]
    fn test_janet_bindings() {
        let result = unsafe { janet_init() };
        assert_eq!(result, 0);

        let env = unsafe { janet_core_env(std::ptr::null_mut()) };
        let code = CString::new(r#"(print "Hello World!")"#)
            .unwrap()
            .as_c_str()
            .as_ptr();
        let file = CString::new("hello-world.janet")
            .unwrap()
            .as_c_str()
            .as_ptr();
        let result = unsafe { janet_dostring(env, code, file, std::ptr::null_mut()) };
        assert_eq!(result, 0);
    }
}
