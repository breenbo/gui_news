use eframe::{
    egui::{
        CentralPanel, Context, FontData, FontDefinitions, Layout, RichText, ScrollArea, Separator,
    },
    epaint::{Color32, FontFamily},
    App,
};

const PADDING: f32 = 10.;
const TITLE_FONT_SIZE: f32 = 30.;
const DESC_FONT_SIZE: f32 = 25.;
const URL_FONT_SIZE: f32 = 20.;
const WHITE: Color32 = Color32::WHITE;
const BLUE: Color32 = Color32::BLUE;

#[derive(Default)]
pub struct Headlines {
    articles: Vec<NewCardData>,
}

impl Headlines {
    pub fn name() -> &'static str {
        "News Headlines"
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            //
            // render title
            //
            ui.add_space(PADDING);
            let formated_title = format!("> {}", &a.title);
            let title_label = RichText::new(formated_title)
                .size(TITLE_FONT_SIZE)
                .color(WHITE);
            ui.label(title_label);
            //
            // render desc
            //
            ui.add_space(PADDING);
            ui.label(RichText::new(&a.desc).size(DESC_FONT_SIZE));
            //
            // render url
            //
            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = BLUE;
            let url_label = RichText::new("Read more...").size(URL_FONT_SIZE);
            ui.with_layout(Layout::right_to_left(eframe::egui::Align::TOP), |ui| {
                ui.hyperlink_to(url_label, &a.url)
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
            ui.add_space(PADDING);
        }
    }
}

impl App for Headlines {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }
}

struct NewCardData {
    title: String,
    desc: String,
    url: String,
}
