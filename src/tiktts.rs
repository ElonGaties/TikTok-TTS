use std::sync::Arc;
use anyhow::Result;
use reqwest::{Url, cookie::Jar};
use reqwest::header::{ACCEPT, CONNECTION, CONTENT_LENGTH};

type TTSResp = crate::json::ttsjson::Root;

const USERAGENT: &str = "com.ss.android.ugc.trill/290504 (Linux; U; Android 13; en; sdk_gphone_x86_64; Build/TE1A.220922.025;tt-ok/3.12.13.1)";

pub struct TTS {
    req_client: reqwest::Client,
    session_id: String,
    api_url: Url
}

impl TTS {
    pub fn new(api_url: &str, session_id: &str) -> Result<Self> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let cookies = format!("sessionid={}", session_id);
        let url = api_url.parse::<Url>()?;
        let host = url.host_str().unwrap_or("tiktokv.com");
        jar.add_cookie_str(&cookies, &format!("{}://{}", url.scheme(), host).parse()?);

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(jar)
            .user_agent(USERAGENT)
            .build()?;

        Ok(Self {
            req_client: client,
            session_id: session_id.to_string(),
            api_url: url
        })
    }

    pub async fn get_tts(&self, text: &str, voice: &str) -> Result<TTSResp> { 
        let req = self.req_client.post(self.api_url.clone())
            .query(&[("text_speaker", voice), ("req_text", text),
                ("speaker_map_type", "1"), ("aid", "1233")])
            .header(CONTENT_LENGTH, 0)
            .header(CONNECTION, "keep-alive")
            .header(ACCEPT, "*/*")
            .send().await?.json().await?;
        Ok(req)
    }
}
