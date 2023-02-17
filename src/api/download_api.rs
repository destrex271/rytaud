use actix_web::{
    post, 
    web, get,
    Responder, HttpRequest
};
use serde::{Serialize, Deserialize};
use ytmp3;

#[derive(Deserialize)]
struct File{
    key: String,
    url: String,
    format: String
}

#[derive(Serialize)]
struct Response{
    file: Option<String>,
    error: Option<String>
}

#[derive(Serialize)]
struct IPResponse{
    ip: std::option::Option<std::net::SocketAddr>,
    msg: String
}

#[get("/")]
async fn home(req: HttpRequest) -> impl Responder{
    web::Json(IPResponse{
        ip: req.peer_addr().to_owned(),
        msg: String::from("Hello There! Welcome to rytaud.")
    })
}

#[post("/download/audio")]
async fn download_vid(req: web::Json<File>) -> impl Responder{
    println!("{:?}", req.url);
    let url : &str = &req.url;
    let val = ytmp3::download(url, &req.key, &req.format).await;
    match val{
        Ok(x) => web::Json(Response{
            file: Some(format!("http://0.0.0.0:8000/audio/{}.mp3",x)),
            error: None
        }),
        Err(x) => web::Json(Response{file: None, error: Some(x.to_string())})
    }
}
