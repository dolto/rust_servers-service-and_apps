use std::{collections::HashMap, env, fs};

use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Error;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}{}", public_path, file_name);

        println!("{full_path}");
        let contents = fs::read_to_string(full_path); // 경로 안에 있는 파일의 내용을 가져옴
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct StaticPageHandler;
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        match s.as_str() {
            "" => HttpResponse::new("200", None, Self::load_file("/index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("/health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("/404.html")),
            },
        }
    }
}
pub struct PageNotFoundHandler;
impl Handler for PageNotFoundHandler {
    fn handle(_: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("/404.html"))
    }
}
pub struct WebServiceHandler;
impl WebServiceHandler {
    fn load_json(file_name: &str) -> Result<Vec<OrderStatus>, Error> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let file_name = file_name.replace("/api", "");
        let full_path = format!("{}{}.json", data_path, file_name);
        println!("{full_path}");
        let json_contents = fs::read_to_string(full_path).unwrap_or("".to_string());
        let orders: Vec<OrderStatus> = serde_json::from_str(json_contents.as_str())?;
        Ok(orders)
    }
}
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;

        match Self::load_json(s) {
            Ok(contents) if contents.len() != 0 => {
                let body = Some(serde_json::to_string(&contents).unwrap());
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("/404.html")),
        }
    }
}
