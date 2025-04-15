//! Integration test for `RR-0478` (stability).
//! Runtime telemetry config fuzz CLI wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0478_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe3, 0xe5];
    let full = relayring::capabilities::rr_0478_runtime_telemetry_config::evaluate(fixture).expect("RR-0478: bulk Runtime telemetry config fuzz CLI wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0478_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0478: stable prefix");
        assert!(partial.consumed <= end, "RR-0478: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0478: full prefix should match bulk checksum");
        }
    }
}
