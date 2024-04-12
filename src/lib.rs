use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// 'run'을 public'으로 마크해야한다.
// 'run'은 더 이상 바이너리 엔트리 포인트가 아니므로, proc-macro 주문 없이
// 내부적으로 await를 사용하지 않는다면 메서드에서 async를 제거한다.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// 메모: web::get() 은 Route::new().guard(guard::Get()) 을 짧게 쓴 것이며, guard는 trait다. trait는 C#으로 표현하면 '기본 구현이 존재하는 interface'다.