/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    frame: Option<egui::TextureHandle>,
    #[serde(skip)]
    frame_buf: egui::ColorImage,
}

impl Default for App {
    fn default() -> Self {
        let width = 1024;
        let height = 768;
        let img = egui::ColorImage::new([width, height], egui::Color32::BLACK);
        Self {
            frame: None,
            frame_buf: img,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ui_size = ui.available_size();

            let texture: &mut egui::TextureHandle = self.frame.get_or_insert_with(|| {
                // Load the texture only once.
                ui.ctx().load_texture("frame_buffer", egui::ColorImage::example(), egui::TextureOptions::NEAREST)
            });

            let width = self.frame_buf.width();
            let height = self.frame_buf.height();
            for x in 0..width {
                for y in 0..height {
                    let r = x * 255 / width;
                    let g = y * 255 / height;
                    let b = 0;
                    self.frame_buf.pixels[x + y * width] = egui::Color32::from_rgb(r as u8, g as u8, b as u8);
                }
            }

            texture.set(self.frame_buf.clone(), egui::TextureOptions::NEAREST);
            let tex_size = texture.size_vec2();
            let scale_factor = (ui_size.x / tex_size.x).min(ui_size.y / tex_size.y);
            let draw_size = tex_size * scale_factor;
            ui.vertical_centered(|ui| {
                ui.image(texture, draw_size);
            });
        });
    }
}
