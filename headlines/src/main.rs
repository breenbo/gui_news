use eframe::{egui::Vec2, run_native, NativeOptions};
use headlines::Headlines;

mod headlines;

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.viewport.inner_size = Some(Vec2::new(540., 960.));

    let _ = run_native(
        Headlines::name(),
        win_options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
