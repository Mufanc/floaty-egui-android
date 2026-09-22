fn main() {
    println!("cargo:rerun-if-env-changed=FLOATY_APK");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
        return;
    }
    let source = std::env::var("FLOATY_APK").expect("FLOATY_APK must point to bridge.apk");
    println!("cargo:rerun-if-changed={source}");
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("bridge.apk");
    std::fs::copy(source, output).expect("failed to embed bridge.apk");
    println!("cargo:rustc-link-arg-bin=floaty-egui=-Wl,--export-dynamic-symbol=JNI_OnLoad");
}
