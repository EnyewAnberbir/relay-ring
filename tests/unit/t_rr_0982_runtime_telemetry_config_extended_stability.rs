//! Integration test for `RR-0982` (stability).
//! Extended: Runtime telemetry config fuzz CLI integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0982_runtime_telemetry_config_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let full = relayring::capabilities::rr_0982_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0982: bulk Extended: Runtime telemetry config fuzz CLI integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0982_runtime_telemetry_config_extended::evaluate(&fixture[..end]).expect("RR-0982: stable prefix");
        assert!(partial.consumed <= end, "RR-0982: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0982: full prefix should match bulk checksum");
        }
    }
}
