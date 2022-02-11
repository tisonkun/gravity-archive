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
use serde_json::{Map, Value};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Actor {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(alias = "type")]
    typ: String,
    site_admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Repository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Actor,
    private: bool,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pushed_at: OffsetDateTime,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    size: i64,
    stargazers_count: i64,
    watchers_count: i64,
    language: Option<String>,
    has_issues: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: i64,
    mirror_url: Option<String>,
    open_issues_count: i64,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    default_branch: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Issue {
    url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    id: i64,
    node_id: String,
    number: i64,
    title: String,
    user: Actor,
    labels: Option<Vec<Label>>,
    state: Option<String>,
    locked: Option<bool>,
    assignee: Option<Actor>,
    assignees: Vec<Actor>,
    milestone: Option<Milestone>,
    comments: i64,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    #[serde(with = "super::option_time_rfc3339")]
    closed_at: Option<OffsetDateTime>,
    body: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    number: i64,
    state: String,
    title: String,
    description: String,
    creator: Actor,
    open_issues: i64,
    closed_issues: i64,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    #[serde(with = "super::option_time_rfc3339")]
    closed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    due_on: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Label {
    id: i64,
    node_id: String,
    description: String,
    url: String,
    name: String,
    color: String,
    default: bool,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssueComment {
    url: String,
    html_url: String,
    issue_url: String,
    id: i64,
    node_id: String,
    user: Actor,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    body: String,
    author_association: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestReviewComment {
    url: String,
    html_url: String,
    pull_request_url: String,
    diff_hunk: String,
    path: String,
    id: i64,
    node_id: String,
    user: Actor,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    body: String,
    author_association: String,
    position: i64,
    original_position: i64,
    commit_id: String,
    original_commit_id: String,
    _links: ReviewCommentLinks,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CommitComment {
    url: String,
    html_url: String,
    id: i64,
    node_id: String,
    user: Actor,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    body: String,
    author_association: String,
    position: Option<i64>,
    line: Option<i64>,
    path: Option<String>,
    commit_id: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct LabelChanges {
    name: Option<ChangedFrom>,
    color: Option<ChangedFrom>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssueChanges {
    title: Option<ChangedFrom>,
    body: Option<ChangedFrom>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ChangedFrom {
    from: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Team {
    name: String,
    id: i64,
    node_id: Option<String>,
    deleted: Option<bool>,
    slug: Option<String>,
    description: Option<String>,
    privacy: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    members_url: Option<String>,
    repositories_url: Option<String>,
    permission: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct InstallationId {
    id: i64,
    node_id: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequest {
    url: String,
    id: i64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    number: i64,
    state: String,
    locked: bool,
    title: String,
    user: Actor,
    body: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    #[serde(with = "super::option_time_rfc3339")]
    closed_at: Option<OffsetDateTime>,
    #[serde(with = "super::option_time_rfc3339")]
    merged_at: Option<OffsetDateTime>,
    merge_commit_sha: Option<String>,
    assignee: Option<Actor>,
    assignees: Vec<Actor>,
    milestone: Option<Milestone>,
    draft: Option<bool>,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    requested_reviewers: Vec<Actor>,
    labels: Vec<Label>,
    head: Ref,
    base: Ref,
    _links: PullRequestLinks,
    merged: Option<bool>,
    mergeable: Option<bool>,
    mergeable_state: Option<String>,
    merged_by: Option<Actor>,
    comments: Option<i64>,
    review_comments: Option<i64>,
    commits: Option<i64>,
    additions: Option<i64>,
    deletions: Option<i64>,
    changed_files: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Link {
    href: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct PullRequestLinks {
    #[serde(alias = "self")]
    this: Link,
    html: Link,
    issue: Link,
    comments: Link,
    review_comments: Link,
    review_comment: Link,
    commits: Link,
    statuses: Link,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ReviewLinks {
    html: Link,
    pull_request: Link,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ReviewCommentLinks {
    #[serde(alias = "self")]
    this: Link,
    html: Link,
    pull_request: Link,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Ref {
    label: String,
    #[serde(alias = "ref")]
    refer: String,
    sha: String,
    user: Actor,
    repo: Repository,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Review {
    id: i64,
    node_id: String,
    author_association: String,
    user: Actor,
    body: Option<String>,
    commit_id: String,
    #[serde(with = "time::serde::rfc3339")]
    submitted_at: OffsetDateTime,
    state: String,
    html_url: String,
    pull_request_url: String,
    _links: ReviewLinks,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckRun {
    id: i64,
    node_id: Option<String>,
    name: String,
    head_sha: String,
    status: String,
    conclusion: Option<String>,
    url: String,
    html_url: String,
    started_at: String,
    completed_at: Option<String>,
    output: CheckRunOutput,
    check_suite: CheckSuite,
    app: App,
    pull_requests: Vec<CheckRunPullRequest>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckRunOutput {
    title: Option<String>,
    summary: Option<String>,
    text: Option<String>,
    annotations_count: i64,
    annotations_url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckSuite {
    id: i64,
    head_branch: String,
    head_sha: String,
    status: String,
    conclusion: Option<String>,
    url: String,
    before: String,
    after: String,
    pull_requests: Vec<CheckRunPullRequest>,
    app: App,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct App {
    id: i64,
    node_id: String,
    owner: Actor,
    name: String,
    description: Option<String>,
    external_url: String,
    html_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckRunPullRequest {
    id: i64,
    url: String,
    number: i64,
    head: CheckRunPullRequestRef,
    base: CheckRunPullRequestRef,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct CheckRunPullRequestRef {
    #[serde(alias = "ref")]
    refer: String,
    sha: String,
    repo: RepoRef,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct RepoRef {
    id: i64,
    url: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Page {
    page_name: String,
    title: String,
    summary: Option<String>,
    action: String,
    sha: String,
    html_url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Organization {
    login: String,
    id: i64,
    node_id: String,
    url: String,
    repos_url: String,
    events_url: String,
    hooks_url: String,
    issues_url: String,
    members_url: String,
    public_members_url: String,
    avatar_url: String,
    description: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct DeployKey {
    id: i64,
    key: String,
    url: String,
    title: String,
    verified: bool,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    read_only: bool,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Deployment {
    url: String,
    id: i64,
    node_id: String,
    sha: String,
    #[serde(alias = "ref")]
    refer: String,
    task: String,
    payload: Map<String, Value>,
    original_environment: String,
    environment: String,
    transient_environment: Option<bool>,
    production_environment: Option<bool>,
    description: Option<String>,
    creator: Actor,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    statuses_url: String,
    repository_url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct DeploymentStatus {
    url: String,
    id: i64,
    node_id: String,
    state: String,
    creator: Actor,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    description: String,
    environment: String,
    environment_url: Option<String>,
    log_url: Option<String>,
    target_url: String,
    deployment_url: String,
    repository_url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Installation {
    id: i64,
    account: Actor,
    repository_selection: String,
    access_tokens_url: String,
    repositories_url: String,
    html_url: String,
    app_id: i64,
    target_id: i64,
    target_type: String,
    permissions: InstallationPermissions,
    events: Vec<String>,
    single_file_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct InstallationPermissions {
    actions: Option<String>,
    administration: Option<String>,
    checks: Option<String>,
    content_references: Option<String>,
    contents: Option<String>,
    deployments: Option<String>,
    discussions: Option<String>,
    emails: Option<String>,
    environments: Option<String>,
    issues: Option<String>,
    members: Option<String>,
    metadata: Option<String>,
    organization_administration: Option<String>,
    organization_events: Option<String>,
    organization_hooks: Option<String>,
    organization_packages: Option<String>,
    organization_plan: Option<String>,
    organization_projects: Option<String>,
    organization_secrets: Option<String>,
    organization_self_hosted_runners: Option<String>,
    organization_user_blocking: Option<String>,
    packages: Option<String>,
    pages: Option<String>,
    pull_requests: Option<String>,
    repository_hooks: Option<String>,
    repository_projects: Option<String>,
    secret_scanning_alerts: Option<String>,
    secrets: Option<String>,
    security_events: Option<String>,
    security_scanning_alert: Option<String>,
    single_file: Option<String>,
    statuses: Option<String>,
    team_discussions: Option<String>,
    vulnerability_alerts: Option<String>,
    workflows: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct InstallationRepository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Hook {
    id: i64,
    #[serde(alias = "type")]
    typ: String,
    name: String,
    active: bool,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    events: Vec<String>,
    config: HookConfig,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct HookConfig {
    content_type: String,
    insecure_ssl: String,
    url: String,
    secret: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Invitation {
    id: i64,
    node_id: String,
    login: String,
    email: Option<String>,
    role: String,
    inviter: Actor,
    team_count: i64,
    invitation_teams_url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Membership {
    url: String,
    state: String,
    role: String,
    organization_url: String,
    user: Actor,
}
