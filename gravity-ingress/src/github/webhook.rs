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

use gravity_model::github::payload::Payload;
use hyper::{Body, Method, Request, StatusCode};

pub struct Error {
    code: StatusCode,
    message: String,
}

impl Error {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub async fn handle_request(req: Request<Body>) -> Result<Payload, Error> {
    if req.method() != Method::POST {
        let method = req.method();

        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            format!("ONLY ACCEPT POST (GOT: {})\n", method),
        ));
    }

    if req.uri().path() != "/webhooks" {
        let path = req.uri().path();
        return Err(Error::new(
            StatusCode::NOT_FOUND,
            format!("404 Not Found (GOT: {})\n", path),
        ));
    }

    let headers = req.headers();
    let content_type = headers.get("content-type").unwrap().to_str().unwrap();
    if content_type != "application/json" {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            format!("ONLY ACCEPT application/json (GOT: {})\n", content_type),
        ));
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

    let payload = match hyper::body::to_bytes(req.into_body()).await {
        Ok(payload) => payload,
        Err(e) => {
            return Err(Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("FAILED GET PAYLOAD BYTES (MSG: {})", e),
            ));
        }
    };

    if let Ok(secret) = std::env::var("WEBHOOK_SECRET") {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(mac) => mac,
            Err(e) => {
                return Err(Error::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("FAILED GET HMAC SHA256 (MSG: {})", e),
                ));
            }
        };
        mac.update(payload.as_ref());
        let result = mac.finalize().into_bytes();
        let result = format!("sha256={}", hex::encode(result));
        if result != signature {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                format!(
                    "SIGNATURE MISMATCH (EXPECT: {}, GOT: {})\n",
                    signature, result
                ),
            ));
        }
    }

    let payload = match Payload::convertor(event.as_str()) {
        None => {
            return Err(Error::new(
                StatusCode::NOT_IMPLEMENTED,
                format!("UNSUPPORTED EVENT (GOT: {})\n", event),
            ));
        }
        Some(convertor) => (convertor)(payload.as_ref()),
    };

    match payload {
        Ok(payload) => Ok(payload),
        Err(e) => Err(Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("FAILED CONVERT PAYLOAD (MSG: {})\n", e),
        )),
    }
}
