extern crate core;

use std::{convert::Infallible, net::SocketAddr};

use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn handle_request(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    if req.method() != Method::POST {
        let method = req.method();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("ONLY ACCEPT POST (GET: {})\n", method)))
            .unwrap());
    }

    if req.uri().path() != "/webhooks" {
        let path = req.uri().path();
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("404 Not Found (GET: {})\n", path)))
            .unwrap());
    }

    let headers = req.headers();
    let content_type = headers.get("content-type").unwrap().to_str().unwrap();
    if content_type != "application/json" {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!(
                "ONLY ACCEPT application/json (GET: {})\n",
                content_type
            )))
            .unwrap());
    }

    let payload = hyper::body::to_bytes(req.into_body()).await?;
    let payload = serde_json::from_slice::<gravity::payload::IssuePayload>(payload.as_ref());
    println!("payload: {:?}", payload);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let make_svc = hyper::service::make_service_fn(|_| async {
        Ok::<_, Infallible>(hyper::service::service_fn(move |req| handle_request(req)))
    });
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = Server::bind(&address).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
