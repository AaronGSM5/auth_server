use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Define an asynchronous handler function for the "/" route
#[get("/")]
async fn hello() -> impl Responder {
    // Return an HTTP 200 OK response with "Hello world!" as the body
    HttpResponse::Ok().body("Hello world!")
}

// Another handler function for a different route
#[get("/greet/{name}")]
async fn greet(name: actix_web::web::Path<String>) -> impl Responder {
    // Extract the 'name' parameter from the URL path and return a greeting
    HttpResponse::Ok().body(format!("Hello {}!", name.into_inner()))
}

#[get("/fuck/{name}")]
async fn fuck(name: actix_web::web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Fuck you {}!", name.into_inner()))
}

// The main function where the server is set up and started
#[actix_web::main] // Marks the main function to be run by the Actix system
async fn main() -> std::io::Result<()> {
    // Create an HTTP server instance
    HttpServer::new(|| {
        // Configure the application instance
        App::new()
            .service(hello)
            .service(greet)
            .service(fuck)
    })
    // Bind the server to the address 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    // Run the server
    .run()
    .await
}