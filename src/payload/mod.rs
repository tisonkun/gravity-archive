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

use getset::Getters;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::payload::model::*;

pub mod model;

#[derive(Debug)]
pub enum Payload {
    Issues(Box<IssuesPayload>),
    IssueComment(Box<IssueCommentPayload>),
    Star(Box<StarPayload>),
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssuesPayload {
    action: String,
    issue: Issue,
    changes: Option<Changes>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssueCommentPayload {
    action: String,
    issue: Issue,
    comment: Comment,
    changes: Option<Changes>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct StarPayload {
    action: String,
    #[serde(with = "option_time_rfc3339")]
    starred_at: Option<OffsetDateTime>,
    repository: Repository,
    sender: Actor,
}

// This trick is originally provided by @dtolnay at:
// https://github.com/serde-rs/serde/issues/1301#issuecomment-394108486
/// A helper module to workaround serde with customize functions for `Option`
/// value.
pub(self) mod option_time_rfc3339 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use time::OffsetDateTime;

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
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "time::serde::rfc3339")] OffsetDateTime);
        let wrapper = Option::deserialize(deserializer)?;
        Ok(wrapper.map(|Wrapper(datetime)| datetime))
    }
}
