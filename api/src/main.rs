// src/main.rs

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use namedivider_rs::divider::basic_name_divider::get_basic_name_divider;
use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;
use serde::{Deserialize, Serialize};

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
async fn divide(division_request: web::Json<DivisionRequest>) -> impl Responder {
    let val_result = validate(&division_request).await;
    match val_result {
        Ok(_) => (),
        Err(err) => return err,
    }
    let divider = get_divider(&division_request.mode);
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

fn get_divider(mode: &String) -> Box<dyn NameDivider> {
    if mode == "basic" {
        let divider =
            get_basic_name_divider(" ".to_string(), true, "kanji_feature".to_string(), false);
        return Box::new(divider);
    } else {
        let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
        return Box::new(divider);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(divide))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
