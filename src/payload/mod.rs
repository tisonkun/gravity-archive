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
use serde_json::Result as SerdeJsonResult;
use time::OffsetDateTime;

use crate::payload::model::*;

pub mod model;

#[derive(Debug)]
pub enum Payload {
    CheckRunEvent(Box<CheckRunEvent>),
    CheckSuiteEvent(Box<CheckSuiteEvent>),
    CommitCommentEvent(Box<CommitCommentEvent>),
    CreateEvent(Box<CreateEvent>),
    IssuesEvent(Box<IssuesEvent>),
    IssueCommentEvent(Box<IssueCommentEvent>),
    PullRequestEvent(Box<PullRequestEvent>),
    PullRequestReviewEvent(Box<PullRequestReviewEvent>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEvent>),
    StarEvent(Box<StarEvent>),
}

type Convertor = for<'a> fn(&'a [u8]) -> SerdeJsonResult<Payload>;

macro_rules! convertor_of {
    ($event:ident) => {{
        fn convert(payload: &[u8]) -> SerdeJsonResult<Payload> {
            serde_json::from_slice::<$event>(payload)
                .map(Box::new)
                .map(Payload::$event)
        }
        Some(convert)
    }};
}

impl Payload {
    pub fn convertor(event: &str) -> Option<Convertor> {
        match event {
            "check_run" => convertor_of!(CheckRunEvent),
            "check_suite" => convertor_of!(CheckSuiteEvent),
            "commit_comment" => convertor_of!(CommitCommentEvent),
            "create" => convertor_of!(CreateEvent),
            "issues" => convertor_of!(IssuesEvent),
            "issues_comment" => convertor_of!(IssueCommentEvent),
            "pull_request" => convertor_of!(PullRequestEvent),
            "pull_request_review" => convertor_of!(PullRequestReviewEvent),
            "pull_request_review_comment" => convertor_of!(PullRequestReviewCommentEvent),
            "star" => convertor_of!(StarEvent),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckRunEvent {
    action: String,
    check_run: CheckRun,
    repository: Repository,
    installation: Option<Installation>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckSuiteEvent {
    action: String,
    check_suite: CheckSuite,
    repository: Repository,
    installation: Option<Installation>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CommitCommentEvent {
    action: String,
    comment: CommitComment,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CreateEvent {
    #[serde(alias = "ref")]
    refer: String,
    ref_type: String,
    master_branch: String,
    description: Option<String>,
    pusher_type: String,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssuesEvent {
    action: String,
    issue: Issue,
    changes: Option<Changes>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssueCommentEvent {
    action: String,
    issue: Issue,
    comment: IssueComment,
    changes: Option<Changes>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestEvent {
    action: String,
    number: i64,
    pull_request: PullRequest,
    label: Option<Label>,
    repository: Repository,
    sender: Actor,
    changes: Option<Changes>,
    assignee: Option<Actor>,
    requested_reviewer: Option<Actor>,
    requested_team: Option<Team>,
    installation: Option<Installation>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewEvent {
    action: String,
    review: Review,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: Installation,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewCommentEvent {
    action: String,
    comment: PullRequestReviewComment,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: Installation,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct StarEvent {
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
