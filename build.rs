use std::env;
use std::path::PathBuf;

fn main() {
    // Link to OBS Studio
    pkg_config::Config::new()
        .atleast_version("28.0.0")
        .probe("libobs")
        .unwrap();

    // Link to OBS Studio UI
    pkg_config::Config::new()
        .atleast_version("28.0.0")
        .probe("libobs-ui")
        .unwrap();

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
        println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libtarkov_price_overlay.dylib");
    } else if target.contains("windows") {
        // Windows doesn't need special handling for shared libraries
    } else {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libtarkov_price_overlay.so");
    }
    
    // Output plugin directory information
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_path = PathBuf::from(out_dir);
        println!("Output directory: {}", out_path.display());
    }
} 