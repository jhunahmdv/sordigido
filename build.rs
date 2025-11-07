use autocxx_build::Builder;
use miette::Result;

fn main() -> Result<()> {
    let include_paths = [
        "/home/ahmdv/projects/rust/sordigido/src/cpp/include",
        "/home/ahmdv/projects/cpp/libdigidocpp/src",
    ];

    // let extra_clang_args = [
    //     "-std=c++17",
    //     "--gcc-toolchain=/usr",
    //     "-stdlib=libstdc++",
    //     "-I/usr/include/c++/13",
    //     "-I/usr/include/x86_64-linux-gnu/c++/13",
    //     "-I/usr/include/c++/13/backward",
    //     "-I/usr/lib/gcc/x86_64-linux-gnu/13/include",
    //     "-I/usr/lib/llvm-18/lib/clang/18/include",
    // ];

    let mut b = Builder::new("src/main.rs", &include_paths)
        // .extra_clang_args(&extra_clang_args)
        .build()?;

    b.flag_if_supported("-std=c++17")
        .file("/home/ahmdv/projects/rust/sordigido/src/cpp/src/bridge_digidoc.cpp")
        .compile("autocxx-demo");

    // Link the prebuilt libdigidocpp.so
    println!("cargo:rustc-link-search=native=/home/ahmdv/projects/cpp/libdigidocpp/build/src");
    println!("cargo:rustc-link-lib=dylib=digidocpp");

    // System dependencies libdigidocpp needs
    println!("cargo:rustc-link-lib=dylib=xerces-c");
    println!("cargo:rustc-link-lib=dylib=xmlsec1");
    println!("cargo:rustc-link-lib=dylib=xmlsec1-openssl");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=z");

    println!("cargo:rerun-if-changed=src/main.rs");
    Ok(())
}
