//! Integration test for `RR-0471` (stability).
//! Runtime telemetry config fuzz CLI export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0471_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0xde];
    let full = relayring::capabilities::rr_0471_runtime_telemetry_config::evaluate(fixture).expect("RR-0471: bulk Runtime telemetry config fuzz CLI export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0471_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0471: stable prefix");
        assert!(partial.consumed <= end, "RR-0471: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0471: full prefix should match bulk checksum");
        }
    }
}
