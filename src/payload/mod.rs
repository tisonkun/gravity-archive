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

use crate::payload::model::*;

pub mod model;

#[derive(Debug)]
pub enum Payload {
    Issues(Box<IssuesPayload>),
    IssueComment(Box<IssueCommentPayload>),
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
