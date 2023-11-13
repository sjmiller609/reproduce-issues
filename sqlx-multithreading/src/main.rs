use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

async fn hello(db_pool: web::Data<PgPool>) -> impl Responder {
    // Execute a simple query against the database
    match sqlx::query!("SELECT 1 as hello")
        .fetch_one(&**db_pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Hello World!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
