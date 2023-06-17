use std::sync::Arc;
use axum::body::StreamBody;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::{Router, Extension};
use axum::routing::{get, post};
use reqwest::{StatusCode, header};
use serde::Deserialize;
use dotenv_codegen::dotenv;
use tokio_util::io::ReaderStream;

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
    let interface = dotenv!("INTERFACE");

    check_ffmpeg().await.unwrap();

    let tts_client = Arc::new(TTS::new(api_url, session_id).unwrap());

    let app = Router::new()
            .route("/", get(|| async { "Hallo" }))
            .route("/api", get(|Extension(tts_client): Extension<Arc<TTS>>| async move { 
                format!("Api: {}", tts_client.api_url.as_str()) 
            }))
            .route("/request", post(anon_request))
            .layer(Extension(tts_client));

    axum::Server::bind(&interface.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct RequestQuery {
    text: String,
    voice: String // Could be a enum, but there are a lot of voices (schizo fox)
}

async fn anon_request(query: Query<RequestQuery>, Extension(tts_client): Extension<Arc<TTS>>) 
        -> Result<([(header::HeaderName, String); 2], StreamBody<ReaderStream<tokio::fs::File>>), AppError> {
    /*let data = format!("{:<30}|{}", &query.text, &query.voice);
    let hash = md5::compute(data);
    let hash_str = format!("{:x}", hash);*/
    
    let tts_res = tts_client.get_tts(&query.text, &query.voice).await?;

    let b64_str = tts_res.data.v_str;
    convert_dfwpm(&b64_str).await?;
    
    let file = tokio::fs::File::open(&"output.dfpwm").await?;

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream".to_string()),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"output.dfpwm\"".to_string(),
        ),
    ];
    
    Ok((headers, body))
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}