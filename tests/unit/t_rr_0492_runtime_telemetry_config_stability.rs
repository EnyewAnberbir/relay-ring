//! Integration test for `RR-0492` (stability).
//! Runtime telemetry config fuzz CLI integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0492_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let full = relayring::capabilities::rr_0492_runtime_telemetry_config::evaluate(fixture).expect("RR-0492: bulk Runtime telemetry config fuzz CLI integrate validator v37");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0492_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0492: stable prefix");
        assert!(partial.consumed <= end, "RR-0492: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0492: full prefix should match bulk checksum");
        }
    }
}
