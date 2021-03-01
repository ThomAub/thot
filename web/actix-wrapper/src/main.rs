use std::time::{Duration, Instant};

use actix_web::{get, middleware::Logger, web, web::ServiceConfig, App, HttpServer, Responder};
use serde::Serialize;

// static start_at: Instant = Instant::now();

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, Option<String>)>) -> impl Responder {
    format!("Hello {:?}! id:{}", name, id)
}

struct AppInfo {
    app_name: &'static str,
    version: &'static str,
    start_at: Instant,
}

#[derive(Serialize)]
struct Healthcheck {
    app_name: &'static str,
    version: &'static str,
    uptime: Duration,
}

#[get("/api/status")]
async fn healthcheck(app_info: web::Data<AppInfo>) -> impl Responder {
    web::Json(Healthcheck {
        app_name: app_info.app_name.clone(),
        version: app_info.version.clone(),
        uptime: app_info.start_at.elapsed(),
    })
}

fn configure_app(app: &mut web::ServiceConfig) {
    app.data(AppInfo {
        app_name: "Thot",
        version: std::env!("CARGO_PKG_VERSION"),
        start_at: Instant::now(),
    })
    .service(index)
    .service(healthcheck);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    env_logger::init();
    HttpServer::new(|| App::new().wrap(Logger::default()).configure(configure_app))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_healthcheck_integration() {
        env_logger::init();
        let mut app =
            test::init_service(App::new().wrap(Logger::default()).configure(configure_app)).await;
        let req = test::TestRequest::get().uri("/api/status").to_request();
        let resp = test::call_service(&mut app, req).await;
        dbg!(&resp);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
