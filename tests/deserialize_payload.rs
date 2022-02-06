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

use gravity::payload::{IssueCommentPayload, IssuesPayload, StarPayload};

#[test]
fn test_issues_closed() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issues-closed.json")?;
    let payload: IssuesPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("closed", payload.action());
    Ok(())
}

#[test]
fn test_issues_edited() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issues-edited.json")?;
    let payload: IssuesPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("edited", payload.action());
    assert!(payload.changes().is_some());
    Ok(())
}

#[test]
fn test_issue_comment_edited() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issue-comment-edited.json")?;
    let payload: IssueCommentPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("edited", payload.action());
    assert!(payload.changes().is_some());
    Ok(())
}

#[test]
fn test_issue_comment_created() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issue-comment-created.json")?;
    let payload: IssueCommentPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("created", payload.action());
    Ok(())
}

#[test]
fn test_issue_comment_deleted() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issue-comment-deleted.json")?;
    let payload: IssueCommentPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("deleted", payload.action());
    Ok(())
}

#[test]
fn test_star_created() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/star_created.json")?;
    let payload: StarPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("created", payload.action());
    Ok(())
}

#[test]
fn test_star_deleted() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/star_deleted.json")?;
    let payload: StarPayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("deleted", payload.action());
    Ok(())
}
