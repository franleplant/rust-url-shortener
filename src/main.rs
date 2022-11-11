use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use harsh::Harsh;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

// creat a new harshid
fn get_id() -> String {
    let harsh = Harsh::builder().salt("salt goes here!").build().unwrap();
    let random_vec = (0..1).map(|_i| rand::random::<u64>()).collect::<Vec<u64>>();
    harsh.encode(&random_vec)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
async fn get_shorten_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    println!("id: {}", id);

    let (url,) = sqlx::query_as::<_, (String,)>("select url from urls where short_id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    println!("url: {}", url);

    HttpResponse::PermanentRedirect()
        .append_header(("Location", url))
        .finish()
}

#[derive(std::fmt::Debug, Deserialize)]
struct ShortenUrl {
    url: String,
}

#[post("/shorturl")]
async fn shorten_url(data: web::Data<AppState>, payload: web::Json<ShortenUrl>) -> impl Responder {
    let id = get_id();
    println!("{:?} {:?}", id, payload);

    sqlx::query("INSERT INTO urls (short_id, url) VALUES ($1, $2)")
        .bind(&id)
        .bind(payload.url.clone())
        .execute(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("http://localhost/{}", id))
}

struct AppState {
    pub db: sqlx::PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    const HOST: &str = "127.0.0.1";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:54321")
        .await
        .unwrap();

    println!("success connecting to the db");

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, 150);

    println!("Starting server at http://{}:{}", HOST, PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(shorten_url)
            .service(get_shorten_url)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
