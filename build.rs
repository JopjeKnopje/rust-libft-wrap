use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("hello")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");


    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=hello");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    let mut build = cc::Build::new();
    let mut build = build
        .include(libdir_path.join("include/"))
        .include(libdir_path.join("src/"));

    let source_files = vec![libdir_path.join("src/hello.c")];

    for file in source_files {
        build = build.file(file);
    }

    build.compile("libft.a");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("hello/include/hello.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings_path = Path::new(&manifest_dir).join("src/bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
