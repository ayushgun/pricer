mod models;
use actix_extensible_rate_limit::{
    backend::memory::InMemoryBackend, backend::SimpleInputFunctionBuilder, RateLimiter,
};
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

// Declare response object
#[derive(Serialize)]
struct Result {
    price: f64,
}

// Add logic to API routes
#[get("/")]
async fn root() -> impl Responder {
    format!("Pricer API is online.")
}

#[get("/call/black_scholes")]
async fn black_scholes_call(data: web::Json<models::BlackScholes>) -> impl Responder {
    // Construct JSON response
    let response = Result {
        price: data.call_price(),
    };

    // Return JSON response and 200 status
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .json(web::Json(response))
}

#[get("/call/binomial")]
async fn binomial_call(data: web::Json<models::Binomial>) -> impl Responder {
    // Construct JSON response
    let response = Result {
        price: data.call_price(),
    };

    // Return JSON response and 200 status
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .json(web::Json(response))
}

#[get("/put/black_scholes")]
async fn black_scholes_put(data: web::Json<models::BlackScholes>) -> impl Responder {
    // Construct JSON response
    let response = Result {
        price: data.put_price(),
    };

    // Return JSON response and 200 status
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .json(web::Json(response))
}

#[get("/put/binomial")]
async fn binomial_put(data: web::Json<models::Binomial>) -> impl Responder {
    // Construct JSON response
    let response = Result {
        price: data.put_price(),
    };

    // Return JSON response and 200 status
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .json(web::Json(response))
}

// Connect services to HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Log start message in console
    println!("Starting API at http://localhost:8000/");

    // Initialize ratelimiter in-memory storage
    let store = InMemoryBackend::builder().build();

    // Initialize HTTP server
    HttpServer::new(move || {
        // Allow 1 request per 1 second
        let input = SimpleInputFunctionBuilder::new(std::time::Duration::from_secs(1), 1)
            .real_ip_key()
            .build();
        let middleware = RateLimiter::builder(store.clone(), input)
            .add_headers()
            .build();

        // Add services and middlewares
        App::new()
            .wrap(middleware)
            .service(root)
            .service(black_scholes_call)
            .service(binomial_call)
            .service(black_scholes_put)
            .service(binomial_put)
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
