use actix_web::HttpResponse;

pub async fn prometheus_metrics() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body("# HELP oxide_requests_total Total requests\n# TYPE oxide_requests_total counter\noxide_requests_total 0\n")
}
