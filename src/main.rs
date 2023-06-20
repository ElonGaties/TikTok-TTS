use std::sync::Arc;
use axum::routing::{get, on, MethodFilter};
use axum::{Router, Extension};
use axum::body::StreamBody;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use reqwest::{StatusCode, header};
use tokio_util::io::ReaderStream;
use serde::Deserialize;
use dotenv::dotenv;

use tik_dfpwm::tiktts::TTS;
use tik_dfpwm::convert::{check_ffmpeg, convert_dfwpm};

    /*let tts = TTS::new("",
                          "").unwrap();

    let data = tts.get_tts("Bruh, hi there", "en_us_002").await.unwrap();

    println!("{:?}", data);*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /*let api_url = dotenv!("API_URL");
    let session_id = dotenv!("SESSION_ID"); // This reads the .env file at build time which is not wanted
    let interface = dotenv!("INTERFACE");*/

    dotenv().ok();

    let api_url = std::env::var("API_URL").expect("API_URL must be set");
    let session_id = std::env::var("SESSION_ID").expect("SESSION_ID must be set"); // This is done at runtime
    let interface = std::env::var("INTERFACE").expect("INTERFACE must be set");

    check_ffmpeg().await.unwrap();

    let tts_client = Arc::new(TTS::new(&api_url, &session_id).unwrap());

    let app = Router::new()
            .route("/", get(|| async { "Hallo" }))
            .route("/api", get(|Extension(tts_client): Extension<Arc<TTS>>| async move { 
                format!("Api: {}", tts_client.api_url.as_str()) 
            }))
            .route(
                "/request", 
                on(MethodFilter::GET | MethodFilter::POST, file_request)
            )
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

async fn file_request(query: Query<RequestQuery>, Extension(tts_client): Extension<Arc<TTS>>) 
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