//! Integration test for `RR-0193` (stability).
//! Journal append seal refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0193_journal_append_seal_refa_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let full = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(fixture).expect("RR-0193: bulk Journal append seal refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(&fixture[..end]).expect("RR-0193: stable prefix");
        assert!(partial.consumed <= end, "RR-0193: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0193: full prefix should match bulk checksum");
        }
    }
}
