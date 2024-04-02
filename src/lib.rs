use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// 'run'을 public'으로 마크해야한다.
// 'run'은 더 이상 바이너리 엔트리 포인트가 아니므로, proc-macro 주문 없이
// async로 마크할 수 있다.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// 메모: web::get() 은 Route::new().guard(guard::Get()) 을 짧게 쓴 것이며, guard는 trait다. trait는 C#으로 표현하면 '기본 구현이 존재하는 interface'다.