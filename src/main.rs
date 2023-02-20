// Modules
mod api;
use ytmp3;
// Actix modules
use actix_files::Files;
use actix_web::{HttpServer, App, middleware::Logger};
use actix_cors::Cors;
// MultiThreading and exit process
use std::thread;
use ctrlc;

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
            .wrap(Logger::default())
            .wrap(
                Cors::permissive()
            )
            .service(Files::new("/audio", ".").show_files_listing())
            .service(api::download_api::download_vid)
            .service(api::download_api::home)
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
