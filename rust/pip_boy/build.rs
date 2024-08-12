// bindgen.exe ..\cxx\include\fwwasm.h -o src\fwwasm.rs --wasm-import-module-name=wiliwasm --use-core --raw-line "#![allow(non_upper_case_globals, unused, dead_code)]"
// bindgen ../cxx/include/fwwasm.h -o src/fwwasm.rs --wasm-import-module-name=wiliwasm --use-core --raw-line '#![allow(non_upper_case_globals, unused, dead_code)]'
//use std::{env, path::PathBuf};

fn main() {
    //todo!();

    // let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let output_rs_file = cargo_dir.join("src").join("fwwasm.rs");
    // println!("cargo:rerun-if-changed={}../cxx/include/fwwasm.h", cargo_dir.to_str().unwrap());
    // println!("cargo:rerun-if-changed={}/build.rs", cargo_dir.to_str().unwrap());

    // // Generate bindings
    // let bindings = bindgen::Builder::default()
    //     .header("../cxx/include/fwwasm.h")
    //     //.allowlist_function("setIO")
    //     //.derive_default(true)
    //     //.derive_debug(true)
    //     //.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     //.wasm_import_module_name("wiliwasm")
    //     //.use_core()
    //     //.raw_line("#![allow(non_upper_case_globals, unused, dead_code, non_camel_case_types)]")
    //     .generate()
    //     .expect("Unable to generate bindings");
    
    // bindings
    //     .write_to_file(&output_rs_file)
    //     .expect("Couldn't write bindings!");
    // println!("cargo:warning={}", output_rs_file.to_str().unwrap());
}