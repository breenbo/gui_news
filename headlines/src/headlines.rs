use eframe::{
    egui::{
        self, Button, Context, FontData, FontDefinitions, Layout, RichText, Separator,
        TopBottomPanel, Window,
    },
    epaint::{Color32, FontFamily},
};
use newsapi::NewsAPI;
use serde::{Deserialize, Serialize};

pub const PADDING: f32 = 10.;
pub const TITLE_FONT_SIZE: f32 = 20.;
const DESC_FONT_SIZE: f32 = 20.;
const URL_FONT_SIZE: f32 = 15.;
const WHITE: Color32 = Color32::WHITE;
const BLUE: Color32 = Color32::BLUE;

#[derive(Default, Serialize, Deserialize)]
pub struct HeadlineConfig {
    pub dark_mode: bool,
    api_key: String,
}

// impl HeadlineConfig {
//     fn new() -> Self {
//         Self {
//             dark_mode: true,
//             api_key: String::new(),
//         }
//     }
// }

#[derive(Default)]
pub struct Headlines {
    articles: Vec<NewCardData>,
    pub config: HeadlineConfig,
    pub api_key_initialized: bool,
}

impl Headlines {
    pub fn new() -> Self {
        // let iter = (0..20).map(|a| NewCardData {
        //     title: format!("title{}", a),
        //     desc: format!("text{}", a),
        //     url: format!("https://www.example.com/{}", a),
        // });
        // use confy to store config on user drive
        let config: HeadlineConfig = confy::load("gui_news", "headlines").unwrap_or_default();

        Self {
            api_key_initialized: !config.api_key.is_empty(),
            articles: vec![],
            config,
        }
    }

    pub fn init(mut self, cc: &eframe::CreationContext<'_>) -> Self {
        fetch_news(&self.config.api_key, &mut self.articles);
        self.configure_font(&cc.egui_ctx);
        //
        self
    }

    pub fn name() -> &'static str {
        "News Headlines"
    }

    fn configure_font(&self, ctx: &Context) {
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

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            //
            // render title
            //
            ui.add_space(PADDING);
            let formated_title = format!("\u{25B6} {}", &a.title);

            if self.config.dark_mode {
                let title_label = RichText::new(formated_title)
                    .size(TITLE_FONT_SIZE)
                    .color(WHITE);
                ui.label(title_label);
            } else {
                let title_label = RichText::new(formated_title)
                    .size(TITLE_FONT_SIZE)
                    .color(Color32::BLACK);
                ui.label(title_label);
            };
            //
            // render desc
            //
            ui.add_space(PADDING);
            ui.label(
                RichText::new(&a.description.clone().unwrap_or("...".to_string()))
                    .size(DESC_FONT_SIZE),
            );
            //
            // render url
            //
            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = BLUE;
            let url_label = RichText::new("Read more \u{2934}").size(URL_FONT_SIZE);
            ui.with_layout(Layout::right_to_left(eframe::egui::Align::TOP), |ui| {
                ui.hyperlink_to(url_label, &a.url)
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
            ui.add_space(PADDING);
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("topPanel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label(RichText::new("\u{269B}").size(DESC_FONT_SIZE));
                });
                //
                ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                    //
                    // let close_btn =
                    //     ui.add(Button::new(RichText::new("\u{1F5D9}").size(DESC_FONT_SIZE)));
                    // if close_btn.clicked() {
                    //     frame.quit();
                    // }
                    //
                    let refresh_btn =
                        ui.add(Button::new(RichText::new("\u{21BA}").size(DESC_FONT_SIZE)));
                    //
                    let theme_btn =
                        ui.add(Button::new(RichText::new("\u{262F}").size(DESC_FONT_SIZE)));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                })
            });
        });
    }

    pub fn render_config(&mut self, ctx: &Context) {
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter your API Key for NYT: ");
            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
            //
            // store api_key on lost focus or enter pressed
            //
            if text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Err(e) = confy::store(
                    "gui_news",
                    "headlines",
                    HeadlineConfig {
                        dark_mode: self.config.dark_mode,
                        api_key: self.config.api_key.to_string(),
                    },
                ) {
                    tracing::error!("Error saving API key: {}", e);
                };

                tracing::info!("API key set");
                self.api_key_initialized = true;
            }
            //
            //
            ui.add_space(10.)
        });
    }
}

fn fetch_news(api_key: &str, articles: &mut Vec<NewCardData>) {
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        let res_articles = response.articles;
        for a in res_articles.iter() {
            let news = NewCardData {
                title: a.title.to_string(),
                url: a.url.to_string(),
                description: Some(
                    a.description
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or("...".to_string()),
                ),
            };
            articles.push(news);
        }
    } else {
        tracing::error!("Failed to fetch news");
    }
}

pub struct NewCardData {
    pub title: String,
    // pub desc: String,
    pub description: Option<String>,
    pub url: String,
}
