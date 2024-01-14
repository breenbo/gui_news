mod headlines;

use eframe::{
    egui::{
        CentralPanel, Context, RichText, ScrollArea, Separator, TopBottomPanel, Ui, Vec2, Visuals,
    },
    epaint::Color32,
    run_native, App, NativeOptions,
};
use headlines::Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        //
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }
        //
        if !self.api_key_initialized {
            self.render_config(ctx);
        } else {
            self.render_top_panel(ctx);
            //
            CentralPanel::default().show(ctx, |ui| {
                //
                render_header(ui, self.config.dark_mode);
                //
                ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    self.render_news_cards(ui);
                });
                //
                render_footer(ctx);
                //
            });
        }
    }
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(headlines::PADDING);
            ui.hyperlink_to("Made with egui", "https://docs.rs/egui/latest/egui/");
            ui.add_space(headlines::PADDING);
        })
    });
}

fn render_header(ui: &mut Ui, dark: bool) {
    ui.vertical_centered(|ui| {
        ui.add_space(headlines::PADDING);
        if dark {
            let head = RichText::new("Headlines")
                .color(Color32::WHITE)
                .size(headlines::TITLE_FONT_SIZE);
            ui.heading(head);
        } else {
            let head = RichText::new("Headlines")
                .color(Color32::BLACK)
                .size(headlines::TITLE_FONT_SIZE);
            ui.heading(head);
        }
        //
        ui.add_space(headlines::PADDING);
        let sep = Separator::default().spacing(20.);
        ui.add(sep);
    });
}

fn main() {
    tracing_subscriber::fmt::init();
    //
    let mut win_options = NativeOptions::default();
    win_options.viewport.inner_size = Some(Vec2::new(540., 960.));

    let headlines = Headlines::new();

    let _ = run_native(
        Headlines::name(),
        win_options,
        Box::new(|cc| Box::new(headlines.init(cc))),
    );
}
