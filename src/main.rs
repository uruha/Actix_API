use actix_web::{get, web, App, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}, id: {}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(index));

    server = if let Some(ip) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(ip)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}