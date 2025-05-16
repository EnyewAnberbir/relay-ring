//! Integration test for `RR-0705` (stability).
//! Extended: Journal append seal implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0705_journal_append_seal_impl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let full = relayring::capabilities::rr_0705_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0705: bulk Extended: Journal append seal implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0705_journal_append_seal_impl_extended::evaluate(&fixture[..end]).expect("RR-0705: stable prefix");
        assert!(partial.consumed <= end, "RR-0705: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0705: full prefix should match bulk checksum");
        }
    }
}
