//! Integration test for `RR-0765` (stability).
//! Extended: Journal OTLP bridge validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0765_journal_otlp_bridge_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let full = relayring::capabilities::rr_0765_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0765: bulk Extended: Journal OTLP bridge validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0765_journal_otlp_bridge_vali_extended::evaluate(&fixture[..end]).expect("RR-0765: stable prefix");
        assert!(partial.consumed <= end, "RR-0765: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0765: full prefix should match bulk checksum");
        }
    }
}
