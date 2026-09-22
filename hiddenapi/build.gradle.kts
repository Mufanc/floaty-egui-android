plugins { alias(libs.plugins.agp.lib) }

android {
    namespace = "xyz.mufanc.floaty.hiddenapi"
    compileSdk = 36
    defaultConfig { minSdk = 29 }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    compileOnly(libs.refine.annotation)
    annotationProcessor(libs.refine.annotation.processor)
}
