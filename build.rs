extern crate meson;

use std::env;
use std::path::PathBuf;

fn main()
{
    // bindgen uses libclang so make sure to install Clang and set the the
    // LIBCLANG_PATH environment variable in order for the build to work:
    // LIBCLANG_PATH=".../libclang.dll"
    #[cfg(windows)]
    {
        use which::which;

        let clang = which("clang").unwrap();
        assert!(clang.exists());

        let libclang = clang.parent().unwrap().join("libclang.dll");
        assert!(libclang.exists());

        env::set_var("LIBCLANG_PATH", libclang);
    }

    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    let build_path = build_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=janet");
    println!("cargo:rustc-link-search=native={}", build_path);

    meson::build("janet", build_path);

    println!("cargo:rerun-if-changed={}/janet.h", build_path);

    let bindings = bindgen::Builder::default()
        .header(format!("{}/janet.h", build_path))
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}
