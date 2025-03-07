//! Integration test for `RR-0213` (stability).
//! Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0213_journal_append_seal_refa_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let full = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("RR-0213: bulk Journal append seal refactor mutator v38");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(&fixture[..end]).expect("RR-0213: stable prefix");
        assert!(partial.consumed <= end, "RR-0213: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0213: full prefix should match bulk checksum");
        }
    }
}
