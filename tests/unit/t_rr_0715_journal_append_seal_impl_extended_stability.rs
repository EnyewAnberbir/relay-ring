//! Integration test for `RR-0715` (stability).
//! Extended: Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0715_journal_append_seal_impl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let full = relayring::capabilities::rr_0715_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0715: bulk Extended: Journal append seal implement pipeline v40");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0715_journal_append_seal_impl_extended::evaluate(&fixture[..end]).expect("RR-0715: stable prefix");
        assert!(partial.consumed <= end, "RR-0715: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0715: full prefix should match bulk checksum");
        }
    }
}
