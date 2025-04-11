//! Integration test for `RR-0459` (stability).
//! Runtime telemetry config fuzz CLI optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0459_runtime_telemetry_config_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let full = relayring::capabilities::rr_0459_runtime_telemetry_config::evaluate(fixture).expect("RR-0459: bulk Runtime telemetry config fuzz CLI optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0459_runtime_telemetry_config::evaluate(&fixture[..end]).expect("RR-0459: stable prefix");
        assert!(partial.consumed <= end, "RR-0459: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0459: full prefix should match bulk checksum");
        }
    }
}
