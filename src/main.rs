use std::sync::{Arc, Mutex, MutexGuard};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;

struct AppState {
    app_name: String,
    counter: Arc<Mutex<i32>>,
}

#[get("/")]
async fn hello(data: Data<AppState>) -> impl Responder {
    let app_name: &String = &data.app_name;
    let counter_clone: Arc<Mutex<i32>> = Arc::clone(&data.counter);
    let counter: MutexGuard<i32> = counter_clone.lock().unwrap();
    let message: String = format!("Hello {app_name} {}", counter).to_string();
    HttpResponse::Ok().body(message)
}

#[get("/state")]
async fn update_state(data: Data<AppState>) -> String {
    let counter_clone: Arc<Mutex<i32>> = Arc::clone(&data.counter);
    let mut mut_counter: MutexGuard<i32> = counter_clone.lock().unwrap();
    *mut_counter += 1;
    format!("Update counter: {}", mut_counter)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state: Data<AppState> = Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: Arc::new(Mutex::new(0)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hello)
            .service(update_state)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
