// Copyright 2022 tison <wander4096@gmail.com>.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{convert::Infallible, net::SocketAddr};

use gravity::payload::PayLoad;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn handle_request(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    if req.method() != Method::POST {
        let method = req.method();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("ONLY ACCEPT POST (GOT: {})\n", method)))
            .unwrap());
    }

    if req.uri().path() != "/webhooks" {
        let path = req.uri().path();
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("404 Not Found (GOT: {})\n", path)))
            .unwrap());
    }

    let headers = req.headers();
    let content_type = headers.get("content-type").unwrap().to_str().unwrap();
    if content_type != "application/json" {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!(
                "ONLY ACCEPT application/json (GOT: {})\n",
                content_type
            )))
            .unwrap());
    }

    let event = headers
        .get("X-GitHub-Event")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let signature = headers
        .get("X-Hub-Signature-256")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let payload = hyper::body::to_bytes(req.into_body()).await?;

    if let Ok(secret) = std::env::var("WEBHOOK_SECRET") {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
        mac.update(payload.as_ref());
        let result = mac.finalize().into_bytes();
        let result = format!("sha256={}", hex::encode(result));
        if result != signature {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "SIGNATURE MISMATCH (EXPECT: {}, GOT: {})\n",
                    signature, result
                )))
                .unwrap());
        }
    }

    let payload = match event.as_str() {
        "issues" => {
            use gravity::payload::IssuePayload;
            serde_json::from_slice::<IssuePayload>(payload.as_ref()).map(PayLoad::Issue)
        }
        event => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body(Body::from(format!("UNSUPPORTED EVENT (GOT: {})\n", event)))
                .unwrap());
        }
    };

    println!("payload: {:?}", payload);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let make_svc = hyper::service::make_service_fn(|_| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle_request))
    });
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = Server::bind(&address).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
