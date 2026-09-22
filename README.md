# floaty-egui-android

An arm64 native Android executable. Its Rust wrapper places an embedded APK in
a memfd and execs `app_process`; Kotlin then loads that same executable as a
native library, creates an always-on-top `SurfaceControl`, and enters Rust to
render egui through EGL/OpenGL ES.

```shell
gradle :bridge:buildNative
adb push target/aarch64-linux-android/release/floaty-egui /data/local/tmp/floaty-egui
adb shell su -c 'chmod 755 /data/local/tmp/floaty-egui && /data/local/tmp/floaty-egui'
```

Pass `--hide-from-screenshots` to omit the layer from screenshots and screen
recordings. Kotlin watches display rotation and resizes the existing buffer
queue; the Rust renderer picks up the new EGL surface dimensions automatically.
