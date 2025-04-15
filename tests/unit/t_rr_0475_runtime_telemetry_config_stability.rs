//! Integration test for `RR-0475` (stability).
//! Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0475_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let full = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(fixture).expect("RR-0475: bulk Runtime telemetry config fuzz CLI implement pipeline v20");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0475: stable prefix");
        assert!(partial.consumed <= end, "RR-0475: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0475: full prefix should match bulk checksum");
        }
    }
}
