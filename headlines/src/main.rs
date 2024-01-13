use eframe::{egui::CentralPanel, run_native, App, NativeOptions};

#[derive(Default)]
struct Headlines;

impl Headlines {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Headlines
    }
}
impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.label("article text"));
    }
}

fn main() {
    let app_name = "Headlines";
    let win_options = NativeOptions::default();
    let _ = run_native(
        app_name,
        win_options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
