use actix_web::{get, web, App, Result, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Message {
    msg: String
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    service: String
}

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let message: Message = Message {
        msg: String::from("Hello from service A")
    };

    Ok(web::Json(message))
}

#[get("/api/v1/{name}")]
async fn print_name(name: web::Path<String>) -> Result<impl Responder> {
    let name: User = User{
        name: format!("Hello {}", name.to_string()),
        service: String::from("A")
    };
    
    Ok(web::Json(name))
}

#[get("/up")]
async fn up() -> Result<impl Responder> {
    let msg: Message = Message {
        msg: String::from("OK")
    };
    
    Ok(web::Json(msg))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(print_name)
            .service(up)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}