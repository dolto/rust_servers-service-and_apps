use std::io::Write;

use http::{httprequest::HttpRequest, httpresponse::HttpResponse};

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            http::httprequest::Method::Get => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match *route.get(1).unwrap_or(&"") {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            // let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            http::httprequest::Method::Post => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match *route.get(1).unwrap_or(&"") {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = PageNotFoundHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
