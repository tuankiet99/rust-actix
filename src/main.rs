use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)] // thư viện để chuyển đổi như JSON.parse convert các params
#[serde(rename_all = "lowercase")]
struct AddParams {
    a: i32,
    b: i32,
}

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

async fn add_numbers(params: web::Query<AddParams>) -> impl Responder {
    let AddParams { a, b } = params.into_inner();
    let sum = a + b;
    let response_text = format!("{} + {} = {}", a, b, sum);
    HttpResponse::Ok().body(response_text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/add", web::get().to(add_numbers))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
