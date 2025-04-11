//! Integration test for `RR-0458` (stability).
//! Runtime telemetry config fuzz CLI wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0458_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xd1];
    let full = relayring::capabilities::rr_0458_runtime_telemetry_config::evaluate(fixture).expect("RR-0458: bulk Runtime telemetry config fuzz CLI wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0458_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0458: stable prefix");
        assert!(partial.consumed <= end, "RR-0458: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0458: full prefix should match bulk checksum");
        }
    }
}
