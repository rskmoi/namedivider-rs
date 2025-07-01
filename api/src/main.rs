// src/main.rs

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use namedivider_rs::divider::basic_name_divider::get_basic_name_divider;
use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    basic_divider: Arc<dyn NameDivider + Send + Sync>,
    gbdt_divider: Arc<dyn NameDivider + Send + Sync>,
}

#[derive(Serialize)]
struct HealthStatus {
    health: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthStatus {
        health: "OK".to_string(),
    })
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct DivisionRequest {
    names: Vec<String>,
    #[serde(default = "default_mode")]
    mode: String,
}

fn default_mode() -> String {
    "basic".to_string()
}

#[derive(Serialize)]
struct DivisionResult {
    divided_names: Vec<ViewDividedName>,
}

#[derive(Serialize)]
struct ViewDividedName {
    family: String,
    given: String,
    separator: String,
    score: f64,
    algorithm: String,
}

async fn validate(division_request: &web::Json<DivisionRequest>) -> Result<(), HttpResponse> {
    if !["basic", "gbdt"].contains(&division_request.mode.as_str()) {
        return Err(HttpResponse::UnprocessableEntity().json("Mode must be 'basic' or 'gbdt'."));
    } else if division_request.names.len() > 1000 {
        return Err(HttpResponse::UnprocessableEntity()
            .json("You can only divide up to 1000 names at a time."));
    }
    Ok(())
}

#[post("/divide")]
async fn divide(
    app_state: web::Data<AppState>,
    division_request: web::Json<DivisionRequest>,
) -> impl Responder {
    let val_result = validate(&division_request).await;
    match val_result {
        Ok(_) => (),
        Err(err) => return err,
    }
    
    let divider = if division_request.mode == "basic" {
        &app_state.basic_divider
    } else {
        &app_state.gbdt_divider
    };
    
    let mut divided_names: Vec<ViewDividedName> = Vec::new();
    for name in &division_request.names {
        let divided_name = divider.divide_name(name);
        divided_names.push(ViewDividedName {
            family: divided_name.family,
            given: divided_name.given,
            separator: divided_name.separator,
            score: divided_name.score,
            algorithm: divided_name.algorithm,
        });
    }
    let division_result = DivisionResult { divided_names };
    HttpResponse::Ok().json(division_result)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let basic_divider = Arc::new(get_basic_name_divider(
        " ".to_string(),
        true,
        "kanji_feature".to_string(),
        false,
    )) as Arc<dyn NameDivider + Send + Sync>;
    
    let gbdt_divider = Arc::new(get_gbdt_name_divider(
        " ".to_string(),
        true,
        "gbdt".to_string(),
    )) as Arc<dyn NameDivider + Send + Sync>;
    
    let app_state = web::Data::new(AppState {
        basic_divider,
        gbdt_divider,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health_check)
            .service(divide)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_divide_basic() {
        let basic_divider = Arc::new(get_basic_name_divider(
            " ".to_string(),
            true,
            "kanji_feature".to_string(),
            false,
        )) as Arc<dyn NameDivider + Send + Sync>;
        
        let gbdt_divider = Arc::new(get_gbdt_name_divider(
            " ".to_string(),
            true,
            "gbdt".to_string(),
        )) as Arc<dyn NameDivider + Send + Sync>;
        
        let app_state = web::Data::new(AppState {
            basic_divider,
            gbdt_divider,
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(divide)
        ).await;

        let req = test::TestRequest::post()
            .uri("/divide")
            .set_json(&DivisionRequest {
                names: vec!["竈門炭治郎".to_string()],
                mode: "basic".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_divide_validation_error() {
        let basic_divider = Arc::new(get_basic_name_divider(
            " ".to_string(),
            true,
            "kanji_feature".to_string(),
            false,
        )) as Arc<dyn NameDivider + Send + Sync>;
        
        let gbdt_divider = Arc::new(get_gbdt_name_divider(
            " ".to_string(),
            true,
            "gbdt".to_string(),
        )) as Arc<dyn NameDivider + Send + Sync>;
        
        let app_state = web::Data::new(AppState {
            basic_divider,
            gbdt_divider,
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(divide)
        ).await;

        let req = test::TestRequest::post()
            .uri("/divide")
            .set_json(&DivisionRequest {
                names: vec!["竈門炭治郎".to_string()],
                mode: "invalid".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 422);
    }
}
