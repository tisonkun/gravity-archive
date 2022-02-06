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
    labels: Vec<Label>,
    state: String,
    locked: bool,
    assignee: Option<Actor>,
    assignees: Vec<Actor>,
    milestone: Option<Milestone>,
    comments: i64,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
    #[serde(with = "option_time_rfc3339")]
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
    #[serde(with = "time::serde::rfc3339")]
    closed_at: OffsetDateTime,
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
pub struct Comment {
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
pub struct Changes {
    title: Option<ChangedFrom>,
    body: Option<ChangedFrom>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct ChangedFrom {
    from: String,
}

// This trick is originally provided by @dtolnay at:
// https://github.com/serde-rs/serde/issues/1301#issuecomment-394108486
/// A helper module to workaround serde with customize functions for `Option`
/// value.
mod option_time_rfc3339 {
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
