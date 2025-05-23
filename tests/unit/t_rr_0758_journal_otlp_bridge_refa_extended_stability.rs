//! Integration test for `RR-0758` (stability).
//! Extended: Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0758_journal_otlp_bridge_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let full = relayring::capabilities::rr_0758_journal_otlp_bridge_refa_extended::evaluate(fixture).expect("RR-0758: bulk Extended: Journal OTLP bridge refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0758_journal_otlp_bridge_refa_extended::evaluate(&fixture[..end]).expect("RR-0758: stable prefix");
        assert!(partial.consumed <= end, "RR-0758: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0758: full prefix should match bulk checksum");
        }
    }
}
