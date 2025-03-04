//! Integration test for `RR-0184` (stability).
//! Journal append seal benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0184_journal_append_seal_benc_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let full = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("RR-0184: bulk Journal append seal benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(&fixture[..end]).expect("RR-0184: stable prefix");
        assert!(partial.consumed <= end, "RR-0184: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0184: full prefix should match bulk checksum");
        }
    }
}
