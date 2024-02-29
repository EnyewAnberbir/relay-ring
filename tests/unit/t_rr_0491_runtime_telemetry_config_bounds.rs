//! Integration test for `RR-0491` (bounds).
//! Runtime telemetry config fuzz CLI export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0491_runtime_telemetry_config_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0491_runtime_telemetry_config::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0491_runtime_telemetry_config::evaluate(fixture).expect("RR-0491 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
