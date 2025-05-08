use actix_web::{App, HttpServer};

// Declare the 'handlers' module. Rust will look for src/handlers/mod.rs.
// This makes the public items within the 'handlers' module accessible.
mod handlers;
// Declare the 'routes' module. Rust will look for src/routes/mod.rs.
// This makes the public items within the 'routes' module accessible.
mod routes;


#[actix_web::main] // Marks the main function to be run by the Actix system
async fn main() -> std::io::Result<()> {
    // Create an HTTP server instance
    HttpServer::new(|| {
        // Configure the application instance
        App::new()
            // Use the configure method to apply the route configuration defined in routes::index
            .configure(routes::index::configure_routes)
            // You can add other configurations here, such as:
            // - Data: .app_data(web::Data::new(AppState { ... }))
            // - Middleware: .wrap(actix_web::middleware::Logger::default())
            // - Other services or scopes
    })
    // Bind the server to the address 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    // Run the server
    .run()
    .await
}
