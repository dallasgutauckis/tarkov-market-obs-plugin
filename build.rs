use std::env;
use std::path::PathBuf;

fn main() {
    // Get host platform
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    
    println!("Target: {}", target);
    println!("Host: {}", host);

    // Try to find OBS Studio libraries
    let obs_lib = pkg_config::Config::new()
        .atleast_version("28.0.0")
        .probe("libobs");

    match obs_lib {
        Ok(_) => {
            println!("Found OBS Studio library");
        }
        Err(e) => {
            println!("cargo:warning=Could not find OBS Studio library: {}", e);
            println!("cargo:warning=Please install OBS Studio development libraries");
            println!("cargo:warning=For Ubuntu/Debian: sudo apt-get install libobs-dev");
            println!("cargo:warning=For Fedora: sudo dnf install obs-studio-devel");
            println!("cargo:warning=For macOS: brew install obs-studio");
            panic!("OBS Studio library not found");
        }
    }

    // Try to find OBS Studio UI library
    let obs_ui_lib = pkg_config::Config::new()
        .atleast_version("28.0.0")
        .probe("libobs-ui");

    match obs_ui_lib {
        Ok(_) => {
            println!("Found OBS Studio UI library");
        }
        Err(e) => {
            println!("cargo:warning=Could not find OBS Studio UI library: {}", e);
            println!("cargo:warning=Please install OBS Studio UI development libraries");
            println!("cargo:warning=For Ubuntu/Debian: sudo apt-get install libobs-dev");
            println!("cargo:warning=For Fedora: sudo dnf install obs-studio-devel");
            println!("cargo:warning=For macOS: brew install obs-studio");
            panic!("OBS Studio UI library not found");
        }
    }

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

    // Tell Cargo to re-run this build script if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/config/mod.rs");
} 