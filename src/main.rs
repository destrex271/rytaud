use actix_web::{HttpServer, Responder, App, post, HttpResponse, web::Json};
use serde::{Deserialize};
use ytmp3;

#[derive(Deserialize)]
struct File{
    url: String
}

#[post("/download_audio")]
async fn download_vid(req: Json<File>) -> impl Responder{
    let url : &str = &req.url;
    let val = ytmp3::download(url, "mp3").await;
    match val{
        Ok(_) => HttpResponse::Ok().body("Downloaded"),
        Err(_) => HttpResponse::InternalServerError().body("Error")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(download_vid)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
