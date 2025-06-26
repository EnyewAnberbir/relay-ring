//! Integration test for `RR-1000` (stability).
//! Extended: Runtime telemetry config fuzz CLI validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_1000_runtime_telemetry_config_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let full = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-1000: bulk Extended: Runtime telemetry config fuzz CLI validate resolver v45");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(&fixture[..end]).expect("RR-1000: stable prefix");
        assert!(partial.consumed <= end, "RR-1000: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-1000: full prefix should match bulk checksum");
        }
    }
}
