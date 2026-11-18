use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("vendor/libft")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=ft");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    let mut build = cc::Build::new();
    let mut build = build
        .include(libdir_path.join("include/"))
        .include(libdir_path.join("src/"));

    let source_files = vec![
        libdir_path.join("src/ascii/ft_isalnum.c"),
        libdir_path.join("src/ascii/ft_isalpha.c"),
        libdir_path.join("src/ascii/ft_isascii.c"),
        libdir_path.join("src/ascii/ft_isdigit.c"),
        libdir_path.join("src/ascii/ft_isprint.c"),
        libdir_path.join("src/convert/ft_itoa.c"),
        libdir_path.join("src/convert/ft_atoi.c"),
        libdir_path.join("src/convert/ft_toupper.c"),
        libdir_path.join("src/convert/ft_tolower.c"),
        libdir_path.join("src/convert/ft_atoi_hex.c"),
        libdir_path.join("src/convert/ft_atol.c"),
        libdir_path.join("src/gnl/get_next_line.c"),
        libdir_path.join("src/gnl/get_next_line_utils.c"),
        libdir_path.join("src/lst/ft_lstnew.c"),
        libdir_path.join("src/lst/ft_lstadd_front.c"),
        libdir_path.join("src/lst/ft_lstsize.c"),
        libdir_path.join("src/lst/ft_lstlast.c"),
        libdir_path.join("src/lst/ft_lstadd_back.c"),
        libdir_path.join("src/lst/ft_lstdelone.c"),
        libdir_path.join("src/lst/ft_lstclear.c"),
        libdir_path.join("src/lst/ft_lstiter.c"),
        libdir_path.join("src/lst/ft_lstmap.c"),
        libdir_path.join("src/mem/ft_bzero.c"),
        libdir_path.join("src/mem/ft_calloc.c"),
        libdir_path.join("src/mem/ft_memchr.c"),
        libdir_path.join("src/mem/ft_memcmp.c"),
        libdir_path.join("src/mem/ft_memcpy.c"),
        libdir_path.join("src/mem/ft_memmove.c"),
        libdir_path.join("src/mem/ft_memset.c"),
        libdir_path.join("src/put/ft_putchar_fd.c"),
        libdir_path.join("src/put/ft_putendl_fd.c"),
        libdir_path.join("src/put/ft_putnbr_fd.c"),
        libdir_path.join("src/put/ft_putstr_fd.c"),
        libdir_path.join("src/str/ft_split.c"),
        libdir_path.join("src/str/ft_strchr.c"),
        libdir_path.join("src/str/ft_striteri.c"),
        libdir_path.join("src/str/ft_strjoin.c"),
        libdir_path.join("src/str/ft_strjoin_free.c"),
        libdir_path.join("src/str/ft_strlcat.c"),
        libdir_path.join("src/str/ft_strlcpy.c"),
        libdir_path.join("src/str/ft_strlen.c"),
        libdir_path.join("src/str/ft_strmapi.c"),
        libdir_path.join("src/str/ft_strncmp.c"),
        libdir_path.join("src/str/ft_strnstr.c"),
        libdir_path.join("src/str/ft_strrchr.c"),
        libdir_path.join("src/str/ft_strtrim.c"),
        libdir_path.join("src/str/ft_substr.c"),
        libdir_path.join("src/str/ft_strdup.c"),
        libdir_path.join("src/str/ft_strlen_2d.c"),
        libdir_path.join("src/str/ft_strfree_2d.c"),
        libdir_path.join("src/str/ft_strisempty.c"),
    ];

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
        .header("vendor/libft/include/libft.h")
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
