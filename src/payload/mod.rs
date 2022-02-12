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
use serde_json::{Map, Result as SerdeJsonResult, Value};
use time::OffsetDateTime;

use crate::payload::model::*;

pub mod model;
pub(self) mod serde_util;

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
    MemberEvent(Box<MemberEvent>),
    MembershipEvent(Box<MembershipEvent>),
    MetaEvent(Box<MetaEvent>),
    MilestoneEvent(Box<MilestoneEvent>),
    OrganizationEvent(Box<OrganizationEvent>),
    OrgBlockEvent(Box<OrgBlockEvent>),
    PageBuildEvent(Box<PageBuildEvent>),
    PingEvent(Box<PingEvent>),
    ProjectCardEvent(Box<ProjectCardEvent>),
    ProjectColumnEvent(Box<ProjectColumnEvent>),
    ProjectEvent(Box<ProjectEvent>),
    PublicEvent(Box<PublicEvent>),
    PullRequestEvent(Box<PullRequestEvent>),
    PullRequestReviewEvent(Box<PullRequestReviewEvent>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEvent>),
    PushEvent(Box<PushEvent>),
    ReleaseEvent(Box<ReleaseEvent>),
    RepositoryEvent(Box<RepositoryEvent>),
    RepositoryVulnerabilityAlertEvent(Box<RepositoryVulnerabilityAlertEvent>),
    StarEvent(Box<StarEvent>),
    StatusEvent(Box<StatusEvent>),
    TeamAddEvent(Box<TeamAddEvent>),
    TeamEvent(Box<TeamEvent>),
    WatchEvent(Box<WatchEvent>),
    WorkflowDispatchEvent(Box<WorkflowDispatchEvent>),
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
            "member" => convertor_of!(MemberEvent),
            "membership" => convertor_of!(MembershipEvent),
            "meta" => convertor_of!(MetaEvent),
            "milestone" => convertor_of!(MilestoneEvent),
            "organization" => convertor_of!(OrganizationEvent),
            "org_block" => convertor_of!(OrgBlockEvent),
            "page_build" => convertor_of!(PageBuildEvent),
            "ping" => convertor_of!(PingEvent),
            "project" => convertor_of!(ProjectEvent),
            "project_card" => convertor_of!(ProjectCardEvent),
            "project_column" => convertor_of!(ProjectColumnEvent),
            "public" => convertor_of!(PublicEvent),
            "pull_request" => convertor_of!(PullRequestEvent),
            "pull_request_review" => convertor_of!(PullRequestReviewEvent),
            "pull_request_review_comment" => convertor_of!(PullRequestReviewCommentEvent),
            "push" => convertor_of!(PushEvent),
            "release" => convertor_of!(ReleaseEvent),
            "repository" => convertor_of!(RepositoryEvent),
            "repository_vulnerability_alert" => convertor_of!(RepositoryVulnerabilityAlertEvent),
            "star" => convertor_of!(StarEvent),
            "status" => convertor_of!(StatusEvent),
            "team" => convertor_of!(TeamEvent),
            "team_add" => convertor_of!(TeamAddEvent),
            "watch" => convertor_of!(WatchEvent),
            "workflow_dispatch" => convertor_of!(WorkflowDispatchEvent),
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
pub struct MemberEvent {
    action: String,
    member: Actor,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct MembershipEvent {
    action: String,
    scope: String,
    member: Actor,
    organization: Organization,
    team: Team,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct MetaEvent {
    action: String,
    hook_id: i64,
    hook: Hook,
    repository: Repository,
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
pub struct OrganizationEvent {
    action: String,
    membership: Option<Membership>,
    invitation: Option<Invitation>,
    organization: Organization,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct OrgBlockEvent {
    action: String,
    blocked_user: Actor,
    organization: Organization,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PageBuildEvent {
    id: i64,
    build: PageBuild,
    repository: Repository,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PingEvent {
    zen: String,
    hook_id: i64,
    hook: Hook,
    repository: Option<Repository>,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ProjectCardEvent {
    action: String,
    changes: Option<ProjectCardChanges>,
    project_card: ProjectCard,
    repository: Option<Repository>,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ProjectColumnEvent {
    action: String,
    changes: Option<ProjectColumnChanges>,
    project_column: ProjectColumn,
    repository: Option<Repository>,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ProjectEvent {
    action: String,
    changes: Option<ProjectChanges>,
    project: Project,
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
pub struct PushEvent {
    #[serde(alias = "ref")]
    refer: String,
    before: String,
    after: String,
    created: bool,
    deleted: bool,
    forced: bool,
    base_ref: Option<String>,
    compare: String,
    repository: Repository,
    pusher: Pusher,
    sender: Actor,
    commits: Vec<Commit>,
    head_commit: Option<Commit>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ReleaseEvent {
    action: String,
    release: Release,
    repository: Repository,
    sender: Actor,
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
pub struct RepositoryVulnerabilityAlertEvent {
    action: String,
    alert: RepositoryVulnerabilityAlert,
    repository: Repository,
    organization: Option<Organization>,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct StarEvent {
    action: String,
    #[serde(with = "serde_util::gh_comp_time")]
    starred_at: Option<OffsetDateTime>,
    repository: Repository,
    sender: Actor,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct StatusEvent {
    id: i64,
    sha: String,
    name: String,
    avatar_url: Option<String>,
    target_url: Option<String>,
    context: String,
    description: Option<String>,
    state: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    repository: Repository,
    sender: Actor,
    organization: Option<Organization>,
    branches: Vec<StatusBranch>,
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

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct WorkflowDispatchEvent {
    #[serde(alias = "ref")]
    refer: String,
    repository: Repository,
    sender: Actor,
    organization: Option<Organization>,
    workflow: String,
    inputs: Map<String, Value>,
}
