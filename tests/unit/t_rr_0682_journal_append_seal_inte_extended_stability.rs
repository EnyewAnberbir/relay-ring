//! Integration test for `RR-0682` (stability).
//! Extended: Journal append seal integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0682_journal_append_seal_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let full = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0682: bulk Extended: Journal append seal integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(&fixture[..end]).expect("RR-0682: stable prefix");
        assert!(partial.consumed <= end, "RR-0682: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0682: full prefix should match bulk checksum");
        }
    }
}
