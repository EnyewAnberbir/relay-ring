//! Integration test for `RR-0978` (stability).
//! Extended: Runtime telemetry config fuzz CLI wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0978_runtime_telemetry_config_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0xdd];
    let full = relayring::capabilities::rr_0978_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0978: bulk Extended: Runtime telemetry config fuzz CLI wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0978_runtime_telemetry_config_extended::evaluate(&fixture[..end]).expect("RR-0978: stable prefix");
        assert!(partial.consumed <= end, "RR-0978: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0978: full prefix should match bulk checksum");
        }
    }
}
