use actix_web::{App, HttpResponse, HttpServer, Responder,  post};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use rand::seq::SliceRandom;

const ANT_HIVE_URL: &str = "0.0.0.0:7070";

// available actions and directions
const ACTIONS: [&'static str; 5] = ["stay", "move", "eat", "take", "put"];
const DIRECTIONS: [&'static str; 4] = ["up", "down", "right", "left"];

// starting listen for http calls on port :7070
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

// sim will make http post request to your bot
#[post("/")]
async fn post_async(req_body: String) -> impl Responder {

    let request: Request = match deserialize_request(&req_body) {
        Err(_) => Request {id: String::from(""), tick: 0, ants: Vec::new()},
        Ok(ok_ants) => ok_ants
    };

    let order_from_ants: Vec<Order> = request.ants.iter()
                                                  .map(|ant| Order {
                                                        ant_id: ant.id,
                                                        act: String::from(*ACTIONS.choose(&mut rand::thread_rng()).unwrap()),   // pick random action from array on line 9
                                                        dir: String::from(*DIRECTIONS.choose(&mut rand::thread_rng()).unwrap())})   // pick random direction from array on line 10
                                                  .collect();

    // prepare response json object                                                
    let responce = Responce {orders: order_from_ants};

    // reading request body with information about map and ants
    let response_body = serde_json::to_string(&responce).unwrap();
    HttpResponse::Ok().body(response_body)
}

// parce json from request body
fn deserialize_request(request: &String ) -> Result<Request> {
    let result: Result<Request> = serde_json::from_str(&*request.to_string());
    result
}

#[derive(Serialize, Deserialize)]
struct Ant {
    pub id: i32,
    pub event: String,
    pub age: i32,
    pub health: i32,
    pub cargo: i32,
    pub point: Point
}

#[derive(Serialize, Deserialize)]
struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Serialize, Deserialize)]
struct Order {
    #[serde(rename = "antId")]
    pub ant_id: i32,
    pub act: String,
    pub dir: String
}

#[derive(Serialize, Deserialize)]
struct Request {
    pub id: String,
    pub tick: i32,
    pub ants: Vec<Ant>
}

#[derive(Serialize, Deserialize)]
struct Responce {
    pub orders: Vec<Order>
}

// this code available at https://github.com/anthive/rust
// to test it localy, submit post request with payload.json using postman or curl
// curl -X 'POST' -d @payload.json http://localhost:7070

// have fun!

