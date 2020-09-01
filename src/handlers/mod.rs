use actix_web::{web, HttpRequest, HttpResponse};

pub fn app_config(config: &mut web::ServiceConfig) {
  let healt_resource = web::resource("/").route(web::get().to(healt));

  config.service(healt_resource);
}

pub async fn healt(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http;
  use actix_web::test;

  #[actix_rt::test]
  async fn test_healt_ok() {
    let req = test::TestRequest::default().to_http_request();
    let res = healt(req).await;
    assert_eq!(res.status(), http::StatusCode::OK);
  }
}
