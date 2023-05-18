use std::sync::Arc;
use axum::extract::Query;
use axum::{Router, Extension};
use axum::routing::{get, post};
use serde::Deserialize;
use dotenv_codegen::dotenv;

use tik_dfpwm::tiktts::TTS;
use tik_dfpwm::convert::{check_ffmpeg, convert_dfwpm};

    /*let tts = TTS::new("",
                          "").unwrap();

    let data = tts.get_tts("Bruh, hi there", "en_us_002").await.unwrap();

    println!("{:?}", data);*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let api_url = dotenv!("API_URL");
    let session_id = dotenv!("SESSION_ID");

    check_ffmpeg().await.unwrap();

    let tts_client = Arc::new(TTS::new(api_url, session_id).unwrap());

    let app = Router::new()
            .route("/", get(|| async { "Hallo" }))
            .route("/api", get(|Extension(state): Extension<Arc<TTS>>| async move { 
                format!("Api: {}", state.api_url.as_str()) 
            }))
            .route("/request", post(request_tts))
            .layer(Extension(tts_client));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct RequestQuery {
    text: String,
    voice: String // Could be a enum, but there are a lot of voices (schizo fox)
}

async fn request_tts(query: Query<RequestQuery>, Extension(state): Extension<Arc<TTS>>) {
    let tts_res = state.get_tts(&query.text, &query.voice).await.unwrap();
    let b64_str = tts_res.data.v_str;
    convert_dfwpm(&b64_str).await.unwrap();

    // TODO: Send file
}