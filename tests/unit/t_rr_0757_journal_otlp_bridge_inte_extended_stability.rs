//! Integration test for `RR-0757` (stability).
//! Extended: Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0757_journal_otlp_bridge_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let full = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("RR-0757: bulk Extended: Journal OTLP bridge integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(&fixture[..end]).expect("RR-0757: stable prefix");
        assert!(partial.consumed <= end, "RR-0757: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0757: full prefix should match bulk checksum");
        }
    }
}
