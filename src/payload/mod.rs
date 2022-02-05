use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::payload::model::*;

pub mod model;

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct IssuePayload {
    action: String,
    issue: Issue,
    repository: Repository,
    sender: Actor,
}
