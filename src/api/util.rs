use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.initializr.v2.3+json"),
    );

    headers.insert(USER_AGENT, HeaderValue::from_static("spring-tui/0.0.1"));

    headers
}

pub fn get_base_url() -> String {
    String::from("https://start.spring.io/")
}
