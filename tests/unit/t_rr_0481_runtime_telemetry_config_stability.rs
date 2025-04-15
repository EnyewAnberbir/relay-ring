//! Integration test for `RR-0481` (stability).
//! Runtime telemetry config fuzz CLI export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0481_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let full = relayring::capabilities::rr_0481_runtime_telemetry_config::evaluate(fixture).expect("RR-0481: bulk Runtime telemetry config fuzz CLI export adapter v26");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0481_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0481: stable prefix");
        assert!(partial.consumed <= end, "RR-0481: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0481: full prefix should match bulk checksum");
        }
    }
}
