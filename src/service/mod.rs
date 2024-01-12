use hyper::{Request, Response, Method, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Full, Empty};
use hyper::body::Bytes;
use std::fs;
use hyper::header::CONTENT_TYPE;

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let html = fs::read_to_string("build/src/index.html").expect("Unable to read file");
            Ok(Response::new(full(html)))
        },
        (&Method::GET, "/style.css") => {
            let _css = fs::read_to_string("style/style.css").expect("Unable to read file");
            let response = Response::builder()
                .header(CONTENT_TYPE, "text/css")
                .body(full(_css))
                .unwrap();
            Ok(response)
        },
        (&Method::GET, "/picture/image_home.jpeg") => {
            let _pict = fs::read("picture/image_home.jpg").expect("Unable to read file");
let response = Response::builder()
                .header(CONTENT_TYPE, "image/jpeg")
                .body(full(_pict))
                .unwrap();
            Ok(response)
        }
        (&Method::GET, "/bundle.js") => {
            let _js = fs::read_to_string("build/dist/bundle.js").expect("Unable to read file");
            let response = Response::builder()
                .header(CONTENT_TYPE, "text/javascript")
                .body(full(_js))
                .unwrap();
            Ok(response)
        },
        (&Method::GET, "/json/user.json") => {
           let _json = fs::read_to_string("json/user.json").expect("Unable to read file");
            Ok(Response::new(full(_json)))
        },
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
