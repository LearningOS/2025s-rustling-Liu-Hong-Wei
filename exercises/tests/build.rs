use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // For tests7
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // For tests8
    println!("cargo:rustc-cfg=feature=\"{}\"", "pass");
}
