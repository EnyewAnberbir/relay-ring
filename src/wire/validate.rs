pub fn validate_frame(data: &[u8], strict: bool) -> Result<Vec<String>, String> {
    let frame = super::decode::decode(data)?;
    let mut issues = Vec::new();
    if strict && frame.head_seq > frame.tail_seq { issues.push("head past tail".into()); }
    Ok(issues)
}
