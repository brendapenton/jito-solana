use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Determine the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir);

    // Check the Rust version and set a flag
    let rust_version_output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to check rustc version");

    let rust_version = String::from_utf8(rust_version_output.stdout).unwrap();
    if rust_version.contains("1.50") {
        println!("cargo:rustc-cfg=rustc_1_50");
    }

    // Example of creating a file in the output directory
    std::fs::write(dest_path.join("config.txt"), "configurations").unwrap();

    // Link to the system libraries if necessary
    #[cfg(target_os = "solana")]
    {
        println!("cargo:rustc-link-lib=getrandom");
    }

    // Add any additional environment variables or configurations here
    println!("cargo:rerun-if-changed=build.rs");
}
