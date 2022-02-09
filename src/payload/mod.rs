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
    Issues(Box<IssuesPayload>),
    IssueComment(Box<IssueCommentPayload>),
    PullRequest(Box<PullRequestPayload>),
    PullRequestReview(Box<PullRequestReviewPayload>),
    PullRequestReviewComment(Box<PullRequestReviewCommentPayload>),
    Star(Box<StarPayload>),
}

type Convertor = for<'a> fn(&'a [u8]) -> SerdeJsonResult<Payload>;

impl Payload {
    pub fn convertor(event: &str) -> Option<Convertor> {
        match event {
            "issues" => {
                fn convert_issues(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<IssuesPayload>(payload)
                        .map(Box::new)
                        .map(Payload::Issues)
                }
                Some(convert_issues)
            }
            "issue_comment" => {
                fn convert_issue_comment(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<IssueCommentPayload>(payload)
                        .map(Box::new)
                        .map(Payload::IssueComment)
                }
                Some(convert_issue_comment)
            }
            "pull_request" => {
                fn convert_pull_request(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<PullRequestPayload>(payload)
                        .map(Box::new)
                        .map(Payload::PullRequest)
                }
                Some(convert_pull_request)
            }
            "pull_request_review" => {
                fn convert_pull_request_review(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<PullRequestReviewPayload>(payload)
                        .map(Box::new)
                        .map(Payload::PullRequestReview)
                }
                Some(convert_pull_request_review)
            }
            "pull_request_review_comment" => {
                fn convert_pull_request_review_comment(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<PullRequestReviewCommentPayload>(payload)
                        .map(Box::new)
                        .map(Payload::PullRequestReviewComment)
                }
                Some(convert_pull_request_review_comment)
            }
            "star" => {
                fn convert_star(payload: &[u8]) -> SerdeJsonResult<Payload> {
                    serde_json::from_slice::<StarPayload>(payload)
                        .map(Box::new)
                        .map(Payload::Star)
                }
                Some(convert_star)
            }
            _ => None,
        }
    }
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
    comment: IssueComment,
    changes: Option<Changes>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestPayload {
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
pub struct PullRequestReviewPayload {
    action: String,
    review: Review,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: Installation,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewCommentPayload {
    action: String,
    comment: PullRequestReviewComment,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: Installation,
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
