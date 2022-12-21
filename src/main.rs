use actix_files::Files;
use actix_web::{HttpServer, Responder, App, post, HttpResponse, web::Json, get};
use serde::{Serialize, Deserialize};
use ytmp3;

#[derive(Deserialize)]
struct File{
    key: String,
    url: String
}

#[derive(Deserialize)]
struct DelJ{
    name: String
}

#[post("/download_audio")]
async fn download_vid(req: Json<File>) -> impl Responder{
    let url : &str = &req.url;
    let val = ytmp3::download(url, &req.key, "mp3").await;
    match val{
        Ok(x) => HttpResponse::Ok().body(format!("/audio/{}.mp3", &req.key)),
        Err(_) => HttpResponse::InternalServerError().body("Error")
    }
}

#[post("/complete")]
async fn delete_vid(req: Json<DelJ>) -> impl Responder{
    println!("Delete");
    let v = ytmp3::delete(&req.name);
    match v{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(Files::new("/audio", ".").show_files_listing())
            .service(delete_vid)
            .service(download_vid)
    })
    .workers(10)
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
