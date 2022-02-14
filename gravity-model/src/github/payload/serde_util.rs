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

// This trick is originally provided by @dtolnay at:
// https://github.com/serde-rs/serde/issues/1301#issuecomment-394108486
/// A helper module to workaround serde with customize functions for `Option`
/// value and GitHub's legacy number format of time.
pub(super) mod gh_comp_time {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::Value;
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    pub fn serialize<S: Serializer>(
        value: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wrapper<'a>(#[serde(with = "time::serde::rfc3339")] &'a OffsetDateTime);
        value.as_ref().map(Wrapper).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        let value: Option<Value> = Option::deserialize(deserializer)?;

        if let Some(value) = value {
            if let Some(ts) = value.as_i64() {
                return OffsetDateTime::from_unix_timestamp(ts)
                    .map(Some)
                    .map_err(D::Error::custom);
            }

            if let Some(t) = value.as_str() {
                return OffsetDateTime::parse(t, &Rfc3339)
                    .map(Some)
                    .map_err(D::Error::custom);
            }
        }

        Ok(None)
    }
}
