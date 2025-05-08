use actix_web::{get, HttpResponse, Responder, web};

// Define an asynchronous handler function for the "/" route
// Make it public so it can be used in routes.rs
#[get("/")]
pub async fn hello() -> impl Responder {
    // Return an HTTP 200 OK response with "Hello world!" as the body
    HttpResponse::Ok().body("Hello world!")
}

// Another handler function for a different route
// Make it public so it can be used in routes.rs
#[get("/greet/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    // Extract the 'name' parameter from the URL path and return a greeting
    HttpResponse::Ok().body(format!("Hello {}!", name.into_inner()))
}

// You would add more handler functions here for other API endpoints.
// These functions contain the logic for processing specific requests.
