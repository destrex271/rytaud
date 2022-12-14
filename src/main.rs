use actix_files::Files;
use actix_web::{HttpServer, Responder, App, post, HttpResponse, web};
use serde::{Serialize, Deserialize};
use ytmp3;
use std::thread;
use ctrlc;

#[derive(Deserialize)]
struct File{
    key: String,
    url: String
}

#[derive(Deserialize)]
struct DelJ{
    name: String
}


#[derive(Serialize)]
struct Response{
    file: Option<String>,
    error: Option<String>
}

#[post("/download_audio")]
async fn download_vid(req: web::Json<File>) -> impl Responder{
    let url : &str = &req.url;
    let val = ytmp3::download(url, &req.key, "mp3").await;
    match val{
        Ok(_) => web::Json(Response{
            file: Some(format!("/audio/{}.mp3",req.key)),
            error: None
        }),
        Err(x) => web::Json(Response{file: None, error: Some(x)})
    }
}


#[post("/complete")]
async fn delete_vid(req: web::Json<DelJ>) -> impl Responder{
    println!("Delete");
    let v = ytmp3::delete(&req.name);
    match v{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let del_ops = thread::spawn(|| {
        ctrlc::set_handler(move || {
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");
        ytmp3::del_service();
    }); 
    println!("Starting server at: {:?}", std::time::SystemTime::now());
    let server = HttpServer::new(||{
        App::new()
            .service(Files::new("/audio", ".").show_files_listing())
            .service(download_vid)
            .service(delete_vid)

    })
    .workers(10)
    .bind(("0.0.0.0", 8000))?   // Setting server to 0.0.0.0 to make it accessible from outside but
                                // 127.0.0.1 is loopback address and can be used only from within
                                // the container
    .run()
    .await;
    del_ops.join().unwrap();
    server
}
