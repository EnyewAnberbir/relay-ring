//! Integration test for `RR-0694` (stability).
//! Extended: Journal append seal benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0694_journal_append_seal_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let full = relayring::capabilities::rr_0694_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0694: bulk Extended: Journal append seal benchmark reporter v19");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0694_journal_append_seal_benc_extended::evaluate(&fixture[..end]).expect("RR-0694: stable prefix");
        assert!(partial.consumed <= end, "RR-0694: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0694: full prefix should match bulk checksum");
        }
    }
}
