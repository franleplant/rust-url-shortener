use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use harsh::Harsh;

// creat a new harshid
fn get_id() -> String {
    let harsh = Harsh::builder().salt("salt goes here!").build().unwrap();
    let random_vec = (0..1).map(|_i| rand::random::<u64>()).collect::<Vec<u64>>();
    harsh.encode(&random_vec)
}

/* fn main() {
    println!("Hello, world! {}", get_id());
    println!("Hello, world! {}", get_id());
    println!("Hello, world! {}", get_id());
} */

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    const HOST: &str = "127.0.0.1";

    println!("Starting server at http://{}:{}", HOST, PORT);

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind((HOST, PORT))?
        .run()
        .await
}
