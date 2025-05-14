//! Integration test for `RR-0692` (stability).
//! Extended: Journal append seal integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0692_journal_append_seal_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let full = relayring::capabilities::rr_0692_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0692: bulk Extended: Journal append seal integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0692_journal_append_seal_inte_extended::evaluate(&fixture[..end]).expect("RR-0692: stable prefix");
        assert!(partial.consumed <= end, "RR-0692: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0692: full prefix should match bulk checksum");
        }
    }
}
