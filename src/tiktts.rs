use std::sync::Arc;
use reqwest::{header, Method, Url, cookie::Jar};

type TTSResp = crate::ttsjson::Root;

const USERAGENT: String = "com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)".parse().unwrap();

pub struct TTS {
    req_client: reqwest::Client,
    session_id: String,
    api_url: Url
}

impl TTS {
    pub fn new(session_id: &str, api_url: &str) -> Self {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let cookies = format!("sessionid={}", session_id);
        let url = api_url.parse::<Url>().unwrap();
        let host = url.domain().unwrap().parse::<Url>().unwrap();
        jar.add_cookie_str(&cookies, &host);

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(jar)
            .user_agent(USERAGENT)
            .build().unwrap();

        Self {
            req_client: client,
            session_id: session_id.to_string(),
            api_url: url
        }
    }

    pub async fn get_tts(&self, text: &str, voice: &str) -> TTSResp {
        self.req_client.request(Method::POST, &self.api_url)
            .query(&[("text_speaker", voice), ("req_text", text),
                     ("speaker_map_type", "0"), ("aid", "1233")])
            .send().await.unwrap().json().await.unwrap()
    }
}
