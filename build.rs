fn main() {
    // Snapdragon 7 Gen 3 (ARM64 v8.2) CPU optimization
    // Cortex-A78 + Cortex-A55 clusters with Adreno 720 GPU
    
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    println!("cargo:rustc-link-arg=-fuse-ld=lld");
    
    // ARM64 v8.2a target features
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=+v8.2a,+neon,+fp-armv8,+crypto,+dotprod -C opt-level=3");
}
