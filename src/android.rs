use std::ffi::{CString, c_void};
use std::ptr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use egui::{Context, RawInput, ViewportId};
use egui_glow::glow::HasContext;
use egui_glow::{Painter, glow};
use khronos_egl::{self as egl, DynamicInstance};

type Egl = DynamicInstance<egl::EGL1_4>;

pub(super) struct SurfaceRenderer {
    egl: Egl,
    display: egl::Display,
    surface: egl::Surface,
    context: egl::Context,
    glow: Arc<glow::Context>,
    painter: Painter,
    egui: Context,
    density_scale: f32,
    started: Instant,
}

impl SurfaceRenderer {
    /// # Safety
    /// `window` must be a valid `ANativeWindow` and stay alive until this value is dropped.
    pub(super) unsafe fn new(window: *mut c_void, density_scale: f32) -> anyhow::Result<Self> {
        let egl = unsafe { DynamicInstance::<egl::EGL1_4>::load_required() }?;
        let display =
            unsafe { egl.get_display(egl::DEFAULT_DISPLAY) }.ok_or(egl::Error::BadDisplay)?;

        egl.initialize(display)?;
        egl.bind_api(egl::OPENGL_ES_API)?;

        let config_attributes = [
            egl::SURFACE_TYPE,
            egl::WINDOW_BIT,
            egl::RENDERABLE_TYPE,
            egl::OPENGL_ES3_BIT,
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::ALPHA_SIZE,
            8,
            egl::NONE,
        ];
        let config = egl
            .choose_first_config(display, &config_attributes)?
            .ok_or(egl::Error::BadConfig)?;
        let context = egl.create_context(
            display,
            config,
            None,
            &[egl::CONTEXT_CLIENT_VERSION, 3, egl::NONE],
        )?;
        let surface = unsafe { egl.create_window_surface(display, config, window, None) }?;

        egl.make_current(display, Some(surface), Some(surface), Some(context))?;

        let _ = egl.swap_interval(display, 1);

        let glow = Arc::new(unsafe {
            glow::Context::from_loader_function(|name| {
                let name = CString::new(name).expect("GL symbol contains NUL");
                egl.get_proc_address(name.to_str().expect("GL symbol is not UTF-8"))
                    .map_or(ptr::null(), |function| function as *const c_void)
            })
        });
        let painter = Painter::new(glow.clone(), "", None, false)?;

        Ok(Self {
            egl,
            display,
            surface,
            context,
            glow,
            painter,
            egui: Context::default(),
            density_scale,
            started: Instant::now(),
        })
    }

    pub(super) fn draw(&mut self, mut ui: impl FnMut(&Context)) -> anyhow::Result<()> {
        let width = self
            .egl
            .query_surface(self.display, self.surface, egl::WIDTH)?;
        let height = self
            .egl
            .query_surface(self.display, self.surface, egl::HEIGHT)?;

        if width <= 0 || height <= 0 {
            std::thread::sleep(Duration::from_millis(16));
            return Ok(());
        }

        let density_scale = self.density_scale;
        let input = RawInput {
            viewport_id: ViewportId::ROOT,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32) / density_scale,
            )),
            time: Some(self.started.elapsed().as_secs_f64()),
            predicted_dt: 1.0 / 60.0,
            ..Default::default()
        };

        self.egui.begin_pass(input);
        ui(&self.egui);

        let mut output = self.egui.end_pass();
        let primitives = self.egui.tessellate(output.shapes, density_scale);

        unsafe {
            self.glow.viewport(0, 0, width, height);
            self.glow.clear_color(0.0, 0.0, 0.0, 0.0);
            self.glow.clear(glow::COLOR_BUFFER_BIT);
        }

        self.painter.paint_and_update_textures(
            [width as u32, height as u32],
            density_scale,
            &primitives,
            &mut output.textures_delta,
        );
        self.egl.swap_buffers(self.display, self.surface)?;

        Ok(())
    }
}

impl Drop for SurfaceRenderer {
    fn drop(&mut self) {
        self.painter.destroy();
        let _ = self.egl.make_current(self.display, None, None, None);
        let _ = self.egl.destroy_surface(self.display, self.surface);
        let _ = self.egl.destroy_context(self.display, self.context);
        let _ = self.egl.terminate(self.display);
    }
}
