use actix_web::{ post, App, HttpResponse, HttpServer, Responder};

const ANT_HIVE_URL: &str = "0.0.0.0:7070";

// CREATED GLOBAL STATIC ARRAYS, NOT TO ALLOCATE DURING EACH REQUEST:
const ACTIONS: [&'static str; 5] = ["stay", "move", "eat", "take", "put"];
const DIRECTIONS: [&'static str; 4] = ["up", "down", "right", "left"];

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


struct Ant {
    pub Id: i32,
    pub Event: String,
    pub Errors: u32,
    pub Age: i32,
    pub Health: i32,
    pub Payload: i32,
    pub Point: Point
}

struct Point {
    pub X: i32,
    pub Y: i32
}

struct Order {
    pub AntId: i32,
    pub Action: String,
    pub Direction: String
}

struct Request {
    pub Id: String,
    pub Tick: i32,
    pub Ants: Vec<Ant>
}