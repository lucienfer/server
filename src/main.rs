use std::net::SocketAddr;
use serde::Serialize;

use http_body_util::{combinators::BoxBody, BodyExt, Full, Empty};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method, StatusCode, Error};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::fs;

// async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     println!("test");
//     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
// }
#[derive(Serialize)]
struct User {
    name: String,
    pwd: String,
}

async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d \"hello world\"`",
        ))),
        (&Method::GET, "/test") => {
            let html = fs::read_to_string("index/test.html").expect("Unable to read file");
            Ok(Response::new(full(html)))
        },
        (&Method::GET, "/wallet") => {
            let html = fs::read_to_string("build/src/index.html").expect("Unable to read file");
            Ok(Response::new(full(html)))
        },
        (&Method::POST, "/authentifier") => {
            let req_byte = req.collect().await?.to_bytes();
            let req_str = String::from_utf8(req_byte.to_vec()).unwrap();
            let parties: Vec<String> = req_str.split('&').map(|s| s.to_string()).collect();
            let _user: Vec<String> = parties[0].split('=').map(|s| s.to_string()).collect();
            let _pwd: Vec<String> = parties[1].split('=').map(|s| s.to_string()).collect();
            let new_user = User{name: _user[1].clone(), pwd: _pwd[1].clone()};
            let json_user = serde_json::to_string(&new_user).unwrap();
            let _ = fs::write("json/user.json", json_user);
            let html = fs::read_to_string("index/authentifier.html").expect("Unable to read file");
            Ok(Response::new(full(html)))
        },
        (&Method::GET, "/json/user.json") => {
           let _json = fs::read_to_string("json/user.json").expect("Unable to read file");
            Ok(Response::new(full(_json)))
        },
        (&Method::POST, "/traiter_mot") => {
            let req_byte = req.collect().await?.to_bytes();
            let req_str = String::from_utf8(req_byte.to_vec()).unwrap();
            let parties: Vec<String> = req_str.split('=').map(|s| s.to_string()).collect();
            let empty_: String = "".to_string();
            let valeur = if parties.len() > 1 { &parties[1] } else { &empty_ };
            // Ok(Response::new(req.into_body().boxed()))
            Ok(Response::new(full(valeur.to_string())))
        },
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(echo))
            .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
