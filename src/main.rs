use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::RwLock;
use actix_web::{post, Responder, App, HttpServer, web, HttpResponse, get};
use dotenv_codegen::dotenv;

use tik_dfpwm::tiktts::TTS;
use tik_dfpwm::convert::check_ffmpeg;

struct RestState {
    tts_client: Arc<RwLock<TTS>>
}

    /*let tts = TTS::new("",
                          "").unwrap();

    let data = tts.get_tts("Bruh, hi there", "en_us_002").await.unwrap();

    println!("{:?}", data);*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_url = dotenv!("API_URL");
    let session_id = dotenv!("SESSION_ID");

    check_ffmpeg().await.unwrap();

    let tts_client = Arc::new(RwLock::new(TTS::new(api_url, session_id).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(RestState {
                tts_client: tts_client.clone()
            }))
            .service(request)
            .service(tes)
    })
    .bind(("127.0.0.1", 132))?
    .run()
    .await
}

#[post("/test")]
async fn tes() -> impl Responder {
    format!("brsh")
}

#[derive(Debug, Deserialize)]
struct QueryData {
    text: String,
    voice: String
}

#[post("/request")]
async fn request(/*data: web::Data<RestState>,*/ queries: web::Query<QueryData>) -> impl Responder {
    format!("{} {}", queries.text, queries.voice)
}
