use colour::{dark_green, grey};
#[cfg(feature = "async")]
use reqwest::Method;

use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2/";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to fetch data from url")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed to convert response to string")]
    ConvertStringFailed(#[from] std::io::Error),
    #[error("Failed to format JSON")]
    FormatFailed(#[from] serde_json::Error),
    #[error("Url error")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    // need to add feature to use reqwest as it is optional
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    code: Option<String>,
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    // pub desc: String,
}

impl NewsAPIResponse {
    pub fn render(&self) {
        for article in &self.articles {
            dark_green!("\n> {}\n", article.title);
            grey!("- {}\n\n", article.url);
        }
    }
}

// pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
//     let response = ureq::get(url)
//         .call()
//         .map_err(NewsApiError::RequestFailed)?
//         .into_string()
//         .map_err(NewsApiError::ConvertStringFailed)?;
//
//     let articles: Articles = serde_json::from_str(&response).map_err(NewsApiError::FormatFailed)?;
//
//     Ok(articles)
// }

pub enum Endpoint {
    TopHeadLines,
}
impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadLines => "top-headlines".to_string(),
        }
    }
}

pub enum Country {
    Us,
    Fr,
}
impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "us".to_string(),
            Self::Fr => "fr".to_string(),
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadLines,
            country: Country::Us,
        }
    }

    //
    // NewsAPI::new().endpoint().country()
    //
    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        //
        // unwrap get the Ok response
        //
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => Ok(response),
            _ => Err(map_response_err(response.code)),
        }
    }

    // make async as optional feature
    #[cfg(feature = "async")]
    pub async fn async_fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();
        let request = client
            .request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsAPIResponse = client
            .execute(request)
            .await?
            .json()
            .await
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => Ok(response),
            _ => Err(map_response_err(response.code)),
        }
    }
}
//
//
fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("API key disabled"),
            _ => NewsApiError::BadRequest("Unknown error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}

pub fn render(articles: &Vec<Article>) {
    for article in articles {
        dark_green!("\n> {}\n", article.title);
        grey!("- {}\n\n", article.url);
    }
}
