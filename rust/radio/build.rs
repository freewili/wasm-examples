// bindgen.exe ..\cxx\include\fwwasm.h -o src\fwwasm.rs --wasm-import-module-name=wiliwasm --use-core --raw-line "#![allow(non_upper_case_globals, unused, dead_code)]"
// bindgen ../cxx/include/fwwasm.h -o src/fwwasm.rs --wasm-import-module-name=wiliwasm --use-core --raw-line '#![allow(non_upper_case_globals, unused, dead_code)]'
use std::{env, path::PathBuf};

fn main() {
    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_full_path = out_path.join("fwwasm.rs");
    println!("cargo:rerun-if-changed={}../../fwwasm/include/fwwasm.h", cargo_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}/build.rs", cargo_dir.to_str().unwrap());

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("../../fwwasm/include/fwwasm.h")
        .wasm_import_module_name("wiliwasm")
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        //.raw_line("#![allow(non_upper_case_globals, unused, dead_code, non_camel_case_types)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-fvisibility=default") // https://github.com/rust-lang/rust-bindgen/issues/1941
        .generate()
        .expect("Unable to generate bindings");
    
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    
    bindings
        .write_to_file(&out_full_path)
        .expect("Couldn't write bindings!");
    println!("cargo:warning={}", out_full_path.to_str().unwrap());
}