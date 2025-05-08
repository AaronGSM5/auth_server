use actix_web::web;

// Import the handlers module from the parent directory (src)
// We need to specify the path from the root of the src directory
use crate::handlers::handlers;

// Public function to configure the API routes
// This function takes a ServiceConfig and adds the defined routes to it.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Register the 'hello' handler from the handlers module
        .service(handlers::hello)
        // Register the 'greet' handler from the handlers module
        .service(handlers::greet);

    // Add more service registrations here for other handlers.
    // This file is responsible for mapping paths to handler functions.
}
