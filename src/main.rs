use eframe::egui::{self};
use maliit::input_method::InputMethod;


fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_fullscreen(true),
        renderer: eframe::Renderer::Glow,
        window_builder: Some(Box::new(|builder| {
            builder.with_active(true).with_fullscreen(true)
        })),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    input_method: Option<InputMethod>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            input_method: InputMethod::new().ok()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(3.0);
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                let text_edit = ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
                if text_edit.clicked() {
                    self.input_method.as_ref().map(|im| im.show());
                }
                if text_edit.lost_focus() {
                    self.input_method.as_ref().map(|im| im.hide());
                }
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;

                let paths = std::fs::read_dir("/run/firejail/mnt/dbus").unwrap();

                for path in paths {
                    println!("Name: {}", path.unwrap().path().display())
                }
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!("../assets/ferris.png"));
        });
    }
}
