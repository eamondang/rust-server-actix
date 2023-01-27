use actix_web::{Responder,get, HttpResponse, post, HttpServer, App, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body + "echo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1")
                     .service(hello)
                     .service(echo))
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
