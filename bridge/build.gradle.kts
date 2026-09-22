import org.gradle.internal.os.OperatingSystem

plugins {
    alias(libs.plugins.agp.app)
    alias(libs.plugins.refine)
}

android {
    namespace = "xyz.mufanc.floaty"
    compileSdk = 36
    ndkVersion = "29.0.14206865"
    defaultConfig {
        applicationId = namespace
        minSdk = 29
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"
        multiDexEnabled = false
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    compileOnly(project(":hiddenapi"))
    implementation(libs.refine.annotation)
    implementation(libs.refine.runtime)
}

tasks.register<Exec>("buildNative") {
    dependsOn("assembleRelease")
    val host = if (OperatingSystem.current().isMacOsX) "darwin-x86_64" else "linux-x86_64"
    val sdk = file(requireNotNull(System.getenv("ANDROID_HOME")) { "ANDROID_HOME is not set" })
    val linker = sdk.resolve("ndk/${android.ndkVersion}/toolchains/llvm/prebuilt/$host/bin/aarch64-linux-android29-clang")
    workingDir = rootDir
    commandLine("cargo", "build", "--release", "--target", "aarch64-linux-android")
    environment("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER", linker)
    environment("FLOATY_APK", layout.buildDirectory.file("outputs/apk/release/bridge-release-unsigned.apk").get().asFile)
}
