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

use hyper::{Body, Request, Response, Server, StatusCode};

async fn handle_request(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    match gravity_ingress::github::webhook::handle_request(req).await {
        Ok(payload) => {
            println!("payload: {:?}", payload);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
        }
        Err(e) => Ok(Response::builder()
            .status(e.code())
            .body(Body::from(e.message().to_string()))
            .unwrap()),
    }
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
