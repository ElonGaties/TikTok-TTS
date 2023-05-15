use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::RwLock;
use actix_web::{post, Responder, App, HttpServer, web, HttpResponse, get};
use dotenv_codegen::dotenv;

use tik_dfpwm::tiktts::TTS;

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

    let tts_client = Arc::new(RwLock::new(TTS::new(api_url, session_id).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(RestState {
                tts_client: tts_client.clone()
            }))
            .service(request)
    })
    .bind(("127.0.0.1", 132))?
    .run()
    .await
}

#[derive(Deserialize)]
struct FormData {
    test: String,
}

#[post("/request")]
async fn request(form: web::Form<FormData>, data: web::Data<RestState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", form.test))
}
