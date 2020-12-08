use actix_web::{ post, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

const ANT_HIVE_URL: &str = "0.0.0.0:7070";
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
    // Here desiarilze
    //let request = &req_body;
    let ants: Vec<Ant> = Vec::new();

    let order_from_ants: Vec<Order> = ants.iter()
                            .map(|x| Order {
                                AntId: x.Id, 
                                Action: String::from(""), 
                                Direction: String::from("")})
                            .collect();

    let responce = Responce {Orders: order_from_ants};
    // Here serialize 

    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize, Deserialize)]
struct Ant {
    pub Id: i32,
    pub Event: String,
    pub Errors: u32,
    pub Age: i32,
    pub Health: i32,
    pub Payload: i32,
    pub Point: Point
}
#[derive(Serialize, Deserialize)]
struct Point {
    pub X: i32,
    pub Y: i32
}
#[derive(Serialize, Deserialize)]
struct Order {
    pub AntId: i32,
    pub Action: String,
    pub Direction: String
}
#[derive(Serialize, Deserialize)]
struct Request {
    pub Id: String,
    pub Tick: i32,
    pub Ants: Vec<Ant>
}

#[derive(Serialize, Deserialize)]
struct Responce {
    pub Orders: Vec<Order>
}