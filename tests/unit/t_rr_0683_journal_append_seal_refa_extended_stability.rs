//! Integration test for `RR-0683` (stability).
//! Extended: Journal append seal refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0683_journal_append_seal_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let full = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0683: bulk Extended: Journal append seal refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(&fixture[..end]).expect("RR-0683: stable prefix");
        assert!(partial.consumed <= end, "RR-0683: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0683: full prefix should match bulk checksum");
        }
    }
}
