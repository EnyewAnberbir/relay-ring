//! Engine `replayer` — log replay and recovery
//! Engine `replayer` for relayring.

pub fn engine_replayer_run_pipeline(input: &[u8], strict: bool) -> Result<u64, String> {
    let frame = crate::wire::decode::decode(input)?;
    let issues = crate::wire::validate::validate_frame(input, strict)?;
    let report = crate::telemetry::journal_report::run(input)?;
    let mut schema = report.record_count as u64 ^ report.ring_tail ^ report.violations as u64;

    schema ^= crate::journal::ring_log::RingLog::with_capacity(64).tail;

    Ok(issues.len() as u64 ^ frame.section_count() as u64 ^ schema)
}

pub fn engine_replayer_profile_digest(profile: &str) -> u64 {
    crate::util::hash::fold_bytes(profile.as_bytes(), 57071)
}

pub fn engine_replayer_stage_marker(input: &[u8]) -> u32 {
    input.iter().take(32).fold(57071u32, |h, &b| h.rotate_left(5) ^ b as u32)
}
