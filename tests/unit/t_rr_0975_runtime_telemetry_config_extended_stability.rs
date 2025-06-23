//! Integration test for `RR-0975` (stability).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0975_runtime_telemetry_config_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let full = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0975: bulk Extended: Runtime telemetry config fuzz CLI implement pipeline v20");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(&fixture[..end]).expect("RR-0975: stable prefix");
        assert!(partial.consumed <= end, "RR-0975: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0975: full prefix should match bulk checksum");
        }
    }
}
