//! Integration test for `RR-0484` (stability).
//! Runtime telemetry config fuzz CLI benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0484_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let full = relayring::capabilities::rr_0484_runtime_telemetry_config::evaluate(fixture).expect("RR-0484: bulk Runtime telemetry config fuzz CLI benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0484_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0484: stable prefix");
        assert!(partial.consumed <= end, "RR-0484: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0484: full prefix should match bulk checksum");
        }
    }
}
