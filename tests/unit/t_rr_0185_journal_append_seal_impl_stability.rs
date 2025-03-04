//! Integration test for `RR-0185` (stability).
//! Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0185_journal_append_seal_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let full = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(fixture).expect("RR-0185: bulk Journal append seal implement pipeline v10");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(&fixture[..end]).expect("RR-0185: stable prefix");
        assert!(partial.consumed <= end, "RR-0185: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0185: full prefix should match bulk checksum");
        }
    }
}
