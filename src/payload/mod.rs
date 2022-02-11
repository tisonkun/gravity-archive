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
    DeleteEvent(Box<DeleteEvent>),
    DeployKeyEvent(Box<DeployKeyEvent>),
    DeploymentEvent(Box<DeploymentEvent>),
    DeploymentStatusEvent(Box<DeploymentStatusEvent>),
    ForkEvent(Box<ForkEvent>),
    GollumEvent(Box<GollumEvent>),
    InstallationEvent(Box<InstallationEvent>),
    InstallationRepositoriesEvent(Box<InstallationRepositoriesEvent>),
    IssuesEvent(Box<IssuesEvent>),
    IssueCommentEvent(Box<IssueCommentEvent>),
    LabelEvent(Box<LabelEvent>),
    MilestoneEvent(Box<MilestoneEvent>),
    PublicEvent(Box<PublicEvent>),
    PullRequestEvent(Box<PullRequestEvent>),
    PullRequestReviewEvent(Box<PullRequestReviewEvent>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEvent>),
    RepositoryEvent(Box<RepositoryEvent>),
    StarEvent(Box<StarEvent>),
    TeamAddEvent(Box<TeamAddEvent>),
    TeamEvent(Box<TeamEvent>),
    WatchEvent(Box<WatchEvent>),
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
            "delete" => convertor_of!(DeleteEvent),
            "deploy_key" => convertor_of!(DeployKeyEvent),
            "deployment" => convertor_of!(DeploymentEvent),
            "deployment_status" => convertor_of!(DeploymentStatusEvent),
            "fork" => convertor_of!(ForkEvent),
            "gollum" => convertor_of!(GollumEvent),
            "installation" => convertor_of!(InstallationEvent),
            "installation_repositories" => convertor_of!(InstallationRepositoriesEvent),
            "issues" => convertor_of!(IssuesEvent),
            "issues_comment" => convertor_of!(IssueCommentEvent),
            "label" => convertor_of!(LabelEvent),
            "milestone" => convertor_of!(MilestoneEvent),
            "public" => convertor_of!(PublicEvent),
            "pull_request" => convertor_of!(PullRequestEvent),
            "pull_request_review" => convertor_of!(PullRequestReviewEvent),
            "pull_request_review_comment" => convertor_of!(PullRequestReviewCommentEvent),
            "repository" => convertor_of!(RepositoryEvent),
            "star" => convertor_of!(StarEvent),
            "team" => convertor_of!(TeamEvent),
            "team_add" => convertor_of!(TeamAddEvent),
            "watch" => convertor_of!(WatchEvent),
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
    installation: Option<InstallationId>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckSuiteEvent {
    action: String,
    check_suite: CheckSuite,
    repository: Repository,
    installation: Option<InstallationId>,
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
pub struct DeleteEvent {
    #[serde(alias = "ref")]
    refer: String,
    ref_type: String,
    pusher_type: String,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct DeployKeyEvent {
    action: String,
    key: DeployKey,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct DeploymentEvent {
    deployment: Deployment,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct DeploymentStatusEvent {
    deployment_status: DeploymentStatus,
    deployment: Deployment,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ForkEvent {
    forkee: Repository,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct GollumEvent {
    pages: Vec<Page>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct InstallationEvent {
    action: String,
    installation: Installation,
    repositories: Option<Vec<InstallationRepository>>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct InstallationRepositoriesEvent {
    action: String,
    installation: Installation,
    repositories_added: Vec<InstallationRepository>,
    repositories_removed: Vec<InstallationRepository>,
    requester: Option<Actor>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssuesEvent {
    action: String,
    issue: Issue,
    changes: Option<IssueChanges>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssueCommentEvent {
    action: String,
    issue: Issue,
    comment: IssueComment,
    changes: Option<IssueChanges>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct LabelEvent {
    action: String,
    label: Label,
    changes: Option<LabelChanges>,
    repository: Repository,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct MilestoneEvent {
    action: String,
    milestone: Milestone,
    repository: Repository,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PublicEvent {
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
    changes: Option<IssueChanges>,
    assignee: Option<Actor>,
    requested_reviewer: Option<Actor>,
    requested_team: Option<Team>,
    installation: Option<InstallationId>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewEvent {
    action: String,
    review: Review,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: InstallationId,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewCommentEvent {
    action: String,
    comment: PullRequestReviewComment,
    pull_request: PullRequest,
    repository: Repository,
    sender: Actor,
    installation: InstallationId,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct RepositoryEvent {
    action: String,
    repository: Repository,
    organization: Option<Organization>,
    sender: Actor,
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

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct TeamAddEvent {
    team: Team,
    repository: Repository,
    organization: Organization,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct TeamEvent {
    action: String,
    team: Team,
    organization: Organization,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct WatchEvent {
    action: String,
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
