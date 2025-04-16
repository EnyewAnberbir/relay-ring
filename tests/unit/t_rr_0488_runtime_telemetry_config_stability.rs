//! Integration test for `RR-0488` (stability).
//! Runtime telemetry config fuzz CLI wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0488_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xed, 0xef];
    let full = relayring::capabilities::rr_0488_runtime_telemetry_config::evaluate(fixture).expect("RR-0488: bulk Runtime telemetry config fuzz CLI wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0488_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0488: stable prefix");
        assert!(partial.consumed <= end, "RR-0488: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0488: full prefix should match bulk checksum");
        }
    }
}
