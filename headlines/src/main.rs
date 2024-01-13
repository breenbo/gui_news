use eframe::{
    egui::{CentralPanel, Context, FontData, FontDefinitions, RichText, ScrollArea, Vec2},
    epaint::FontFamily,
    run_native, App, NativeOptions,
};

#[derive(Default)]
struct Headlines {
    articles: Vec<NewCardData>,
}

impl Headlines {
    fn name() -> &'static str {
        "News Headlines"
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let iter = (0..20).map(|a| NewCardData {
            title: format!("title{}", a),
            desc: format!("text{}", a),
            url: format!("https://www.example.com/{}", a),
        });
        Self::configure_font(&cc.egui_ctx);

        Self {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_font(ctx: &Context) {
        // create font object
        let mut fonts_def = FontDefinitions::default();
        // load the font from file
        fonts_def.font_data.insert(
            "courrier_prime".to_owned(),
            FontData::from_static(include_bytes!("../courrier_prime.ttf")),
        );
        // set size of fonts
        fonts_def
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "courrier_prime".to_owned());
        // load the font in the context object
        ctx.set_fonts(fonts_def);
    }
}

impl App for Headlines {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for a in &self.articles {
                    ui.label(RichText::new(&a.title).size(35.));
                    ui.label(RichText::new(&a.desc).size(20.));
                    ui.label(RichText::new(&a.url).size(20.));
                }
            });
        });
    }
}

struct NewCardData {
    title: String,
    desc: String,
    url: String,
}

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.viewport.inner_size = Some(Vec2::new(540., 960.));

    let _ = run_native(
        Headlines::name(),
        win_options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
