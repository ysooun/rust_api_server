use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(main_page))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn main_page(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(pool.as_ref())
        .await;

    match result {
        Ok(users) => {
            let names: Vec<String> = users.iter()
                .map(|user| user.username.clone())
                .collect();

            let response = format!("Main 페이지입니다. 사용자: {:?}", names);
            HttpResponse::Ok()
                .content_type("text/plain; charset=utf-8")
                .body(response)
        }
        Err(e) => {
            eprintln!("데이터베이스 오류: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
