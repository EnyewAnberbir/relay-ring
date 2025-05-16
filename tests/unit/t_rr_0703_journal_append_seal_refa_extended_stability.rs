//! Integration test for `RR-0703` (stability).
//! Extended: Journal append seal refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0703_journal_append_seal_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let full = relayring::capabilities::rr_0703_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0703: bulk Extended: Journal append seal refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0703_journal_append_seal_refa_extended::evaluate(&fixture[..end]).expect("RR-0703: stable prefix");
        assert!(partial.consumed <= end, "RR-0703: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0703: full prefix should match bulk checksum");
        }
    }
}
