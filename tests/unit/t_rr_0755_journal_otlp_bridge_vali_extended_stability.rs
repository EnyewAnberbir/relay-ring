//! Integration test for `RR-0755` (stability).
//! Extended: Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0755_journal_otlp_bridge_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let full = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0755: bulk Extended: Journal OTLP bridge validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(&fixture[..end]).expect("RR-0755: stable prefix");
        assert!(partial.consumed <= end, "RR-0755: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0755: full prefix should match bulk checksum");
        }
    }
}
