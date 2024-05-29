use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web, Error, get};
use reqwest;
use reqwest::Client;

#[get("/{video_id}")]
async fn proxy_image(video_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let url = format!("https://img.youtube.com/vi/{}/0.jpg", video_id);
    let client = Client::new();
    match client.get(&url).send().await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => Ok(HttpResponse::Ok().body(bytes)),
            Err(_) => Ok(HttpResponse::InternalServerError().body("")),
        },
        Err(_) => Ok(HttpResponse::InternalServerError().body("")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET"])
                    .allow_any_origin(),
            )
            .service(proxy_image)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
