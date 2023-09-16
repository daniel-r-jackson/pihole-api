use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/enable")]
async fn enable(req_body: String) -> impl Responder {
    let output = Command::new("pihole")
        .arg("enable")
        .output()
        .expect("Failed to execute command");
    println!("pihole enabled\nreq body: {}", req_body);
    HttpResponse::Ok().body(output.stdout)
}

#[post("/disable")]
async fn disable(req_body: String) -> impl Responder {
    let output = Command::new("pihole")
        .arg("disable")
        .arg("5m")
        .output()
        .expect("Failed to execute command");
    println!("pihole disabled 5m\nreq body: {}", req_body);
    HttpResponse::Ok().body(output.stdout)
}

#[post("/restart")]
async fn restart(req_body: String) -> impl Responder {
    let output = Command::new("pihole")
        .arg("restartdns")
        .output()
        .expect("Failed to execute command");
    println!("pihole dns restarting\nreq body: {}", req_body);
    HttpResponse::Ok().body(output.stdout)

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(disable)
            .service(enable)
            .service(restart)
    })
    .bind(("10.0.4.123", 7878))?
    .run()
    .await
}
