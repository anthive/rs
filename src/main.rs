use actix_web::{ post, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use serde_json::Result;

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
    let ants = get_ant_vector(&req_body).unwrap();

    let order_from_ants: Vec<Order> = ants.iter()
                            .map(|x| Order {
                                antId: x.id,
                                action: String::from(""),
                                direction: String::from("")})
                            .collect();

    let responce = Responce {orders: order_from_ants};

    // Here serialize
    let response_body = serde_json::to_string(&responce).unwrap();

    HttpResponse::Ok().body(response_body)
}


fn get_ant_vector(request: &String ) -> Result<Vec<Ant>> {
    let ants: Vec<Ant> = serde_json::from_str(&*request.to_string())?;
    Ok(ants)
}

#[derive(Serialize, Deserialize)]
struct Ant {
    pub id: i32,
    pub event: String,
    pub errors: u32,
    pub age: i32,
    pub health: i32,
    pub payload: i32,
    pub point: Point
}

#[derive(Serialize, Deserialize)]
struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Serialize, Deserialize)]
struct Order {
    pub antId: i32,
    pub action: String,
    pub direction: String
}

#[derive(Serialize, Deserialize)]
struct Request {
    pub id: String,
    pub rick: i32,
    pub ants: Vec<Ant>
}

#[derive(Serialize, Deserialize)]
struct Responce {
    pub orders: Vec<Order>
}