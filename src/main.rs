use actix_web::{ post, App, HttpResponse, HttpServer, Responder};

const ANT_HIVE_URL: &str = "localhost:7070";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post_async)
    })
    .bind(ANT_HIVE_URL)?
    .run()
    .await
}

#[post("/")]
async fn post_async(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}