//! Integration test for `RR-0215` (stability).
//! Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0215_journal_append_seal_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let full = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("RR-0215: bulk Journal append seal implement pipeline v40");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(&fixture[..end]).expect("RR-0215: stable prefix");
        assert!(partial.consumed <= end, "RR-0215: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0215: full prefix should match bulk checksum");
        }
    }
}
