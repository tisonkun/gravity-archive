use gravity::payload::IssuePayload;

#[test]
fn test_issue_closed() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("tests/testdata/issue-closed.json")?;
    let payload: IssuePayload = serde_json::from_str(contents.as_str())?;
    assert_eq!("closed", payload.action());
    Ok(())
}
