use std::ffi::c_void;

use jni::objects::JObject;
use jni::sys::{JNI_ERR, JNI_VERSION_1_6, jint};
use jni::{EnvUnowned, JavaVM, NativeMethod, Outcome, jni_str};

mod android;

use self::android::SurfaceRenderer;

#[unsafe(export_name = "JNI_OnLoad")]
/// # Safety
/// `vm` must be the valid Java VM pointer supplied by the JNI runtime.
pub unsafe extern "system" fn main(vm: *mut jni::sys::JavaVM, _reserved: *mut c_void) -> jint {
    let vm = unsafe { JavaVM::from_raw(vm) };
    let method = unsafe {
        NativeMethod::from_raw_parts(
            jni_str!("render"),
            jni_str!("(Landroid/view/Surface;F)V"),
            native_render as *mut c_void,
        )
    };

    match vm.attach_current_thread(|env| unsafe {
        env.register_native_methods(jni_str!("xyz/mufanc/floaty/Main"), &[method])
    }) {
        Ok(()) => JNI_VERSION_1_6,
        Err(_) => JNI_ERR,
    }
}

extern "system" fn native_render<'local>(
    mut env: EnvUnowned<'local>,
    _object: JObject<'local>,
    surface: JObject<'local>,
    density_scale: f32,
) {
    match env
        .with_env(|env| render(env, surface, density_scale))
        .into_outcome()
    {
        Outcome::Ok(()) => {}
        Outcome::Err(_) | Outcome::Panic(_) => std::process::exit(1),
    }
}

fn render(env: &mut jni::Env<'_>, surface: JObject<'_>, density_scale: f32) -> anyhow::Result<()> {
    android_logger::init_once(android_logger::Config::default().with_tag("FloatyEgui"));

    let window = unsafe {
        ndk_sys::ANativeWindow_fromSurface(env.get_raw().cast(), surface.as_raw().cast())
    };

    anyhow::ensure!(!window.is_null(), "ANativeWindow_fromSurface returned null");

    let result =
        unsafe { SurfaceRenderer::new(window.cast(), density_scale) }.and_then(|mut renderer| {
            loop {
                renderer.draw(|context| {
                    egui::Area::new(egui::Id::new("floaty-egui"))
                        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                        .show(context, |ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgba_unmultiplied(12, 20, 36, 238))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(44, 67, 96)))
                                .corner_radius(18)
                                .inner_margin(22)
                                .shadow(egui::epaint::Shadow {
                                    offset: [0, 8],
                                    blur: 24,
                                    spread: 2,
                                    color: egui::Color32::from_black_alpha(100),
                                })
                                .show(ui, |ui| {
                                    ui.set_width(248.0);
                                    ui.horizontal(|ui| {
                                        let (dot, _) = ui.allocate_exact_size(
                                            egui::vec2(9.0, 9.0),
                                            egui::Sense::hover(),
                                        );
                                        ui.painter().circle_filled(
                                            dot.center(),
                                            4.5,
                                            egui::Color32::from_rgb(64, 224, 190),
                                        );
                                        ui.label(
                                            egui::RichText::new("FLOATY")
                                                .size(12.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(133, 154, 183)),
                                        );
                                    });
                                    ui.add_space(12.0);
                                    ui.label(
                                        egui::RichText::new("Overlay ready")
                                            .size(25.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(240, 246, 255)),
                                    );
                                    ui.add_space(5.0);
                                    ui.label(
                                        egui::RichText::new("Rust  ·  egui  ·  EGL")
                                            .size(13.0)
                                            .monospace()
                                            .color(egui::Color32::from_rgb(116, 139, 169)),
                                    );
                                    ui.add_space(17.0);
                                    let (track, _) = ui.allocate_exact_size(
                                        egui::vec2(ui.available_width(), 3.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(
                                        track,
                                        2.0,
                                        egui::Color32::from_rgb(28, 43, 64),
                                    );
                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_size(
                                            track.min,
                                            egui::vec2(64.0, track.height()),
                                        ),
                                        2.0,
                                        egui::Color32::from_rgb(64, 224, 190),
                                    );
                                });
                        });
                })?;
            }
        });

    unsafe { ndk_sys::ANativeWindow_release(window) };
    result
}
