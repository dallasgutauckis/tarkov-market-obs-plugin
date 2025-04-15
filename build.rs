use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to re-run this build script if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/config/mod.rs");
    
    // Get host platform
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    
    println!("Target: {}", target);
    println!("Host: {}", host);
    
    // Set platform-specific linker options
    if target.contains("apple") || target.contains("darwin") {
        // macOS doesn't use -soname
        println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libtarkuck.dylib");
    } else if target.contains("windows") {
        // Windows doesn't need special handling for shared libraries
    } else {
        // Linux and others use -soname
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libtarkov-price-overlay.so");
    }
    
    // Output plugin directory information
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_path = PathBuf::from(out_dir);
        println!("Output directory: {}", out_path.display());
    }
} 