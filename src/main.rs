use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    get, web, App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct GreetParameters {
    name: String,
}

#[get("/greet")]
async fn hi(info: web::Query<GreetParameters>) -> String {
    format!("Welcome {} !", info.name)
}

fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = actix_web::Error,
    >,
> {
    App::new().service(hello).service(hi)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
        .unwrap();

    let address = "0.0.0.0";

    println!("Running on {address}:{port}");

    HttpServer::new(create_app)
        .bind((address, port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};
    use std::str;

    #[actix_web::test]
    async fn test_hello_world() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::default().to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_greet_error() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::default().uri("/greet").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_greet_ok() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::default()
            .uri("/greet?name=Salah")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(str::from_utf8(body.as_ref()).unwrap(), "Welcome Salah !");
    }
}
