use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{env, time};
use std::sync::{Arc, Mutex};
use tokio;

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

#[get("/lively")]
pub async fn background_threads_running(
    background_threads: web::Data<Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>>,
) -> impl Responder {
    let background_threads = match background_threads.lock() {
        Ok(threads) => threads,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("Failed to check if background tasks are running.")
        }
    };
    for thread in background_threads.iter() {
        if thread.is_finished() {
            return HttpResponse::InternalServerError()
                .body("One or more background tasks are not running.");
        }
    }
    HttpResponse::Ok().json("ok")
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

    let background_threads: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>> =
        Arc::new(Mutex::new(Vec::new()));

    let db_pool_clone = db_pool.clone();
    let bg_thread = tokio::spawn(async move {
        loop {
            match sqlx::query!("SELECT pg_sleep(5)")
                .fetch_one(&db_pool_clone)
                .await
            {
                Ok(_) => println!("Background query executed successfully."),
                Err(e) => eprintln!("Background query failed: {:?}", e),
            }
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    });

    background_threads.lock().unwrap().push(bg_thread);
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(background_threads.clone()))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
