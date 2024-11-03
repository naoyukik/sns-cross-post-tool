fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        println!("cargo:rustc-env=CARGO_PROFILE={}", profile);
        let log_level = match profile.as_str() {
            "debug" => "debug",
            "bench" => "info",
            "release" => "info",
            _ => "info"
        };
        println!("cargo:rustc-env=RUST_LOG={}", log_level);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
