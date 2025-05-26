//! Integration test for `RR-0761` (stability).
//! Extended: Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0761_journal_otlp_bridge_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let full = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0761: bulk Extended: Journal OTLP bridge extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(&fixture[..end]).expect("RR-0761: stable prefix");
        assert!(partial.consumed <= end, "RR-0761: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0761: full prefix should match bulk checksum");
        }
    }
}
