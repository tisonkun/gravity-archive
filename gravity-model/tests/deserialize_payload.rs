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
use serde_json::Value;

#[test]
fn test_specification() -> anyhow::Result<()> {
    let specs = std::fs::read_to_string("tests/testdata/specification.json")?;
    let specs = serde_json::from_str::<Value>(specs.as_str())?;
    for spec in specs.as_array().unwrap() {
        let spec = spec.as_object().unwrap();

        let event_type = spec.get("name").and_then(Value::as_str).unwrap();
        let convertor = match Payload::convertor(event_type) {
            None => continue,
            Some(convertor) => convertor,
        };

        println!("testing event type {}...", event_type);

        let examples = spec.get("examples").and_then(Value::as_array).unwrap();
        for example in examples {
            let payload = (convertor)(example.to_string().as_bytes());
            assert!(
                payload.is_ok(),
                "cannot parse supported event type {} with:\n\treason:  {}\n\tpayload: {}\n",
                event_type,
                payload.unwrap_err(),
                example,
            );
        }
    }
    Ok(())
}
