//! Integration test for `RR-0992` (stability).
//! Extended: Runtime telemetry config fuzz CLI integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0992_runtime_telemetry_config_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let full = relayring::capabilities::rr_0992_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0992: bulk Extended: Runtime telemetry config fuzz CLI integrate validator v37");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0992_runtime_telemetry_config_extended::evaluate(&fixture[..end]).expect("RR-0992: stable prefix");
        assert!(partial.consumed <= end, "RR-0992: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0992: full prefix should match bulk checksum");
        }
    }
}
