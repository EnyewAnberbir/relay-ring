//! Integration test for `RR-0756` (stability).
//! Extended: Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0756_journal_otlp_bridge_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let full = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(fixture).expect("RR-0756: bulk Extended: Journal OTLP bridge export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(&fixture[..end]).expect("RR-0756: stable prefix");
        assert!(partial.consumed <= end, "RR-0756: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0756: full prefix should match bulk checksum");
        }
    }
}
