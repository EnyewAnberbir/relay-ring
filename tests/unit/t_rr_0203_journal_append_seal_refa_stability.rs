//! Integration test for `RR-0203` (stability).
//! Journal append seal refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0203_journal_append_seal_refa_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let full = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(fixture).expect("RR-0203: bulk Journal append seal refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(&fixture[..end]).expect("RR-0203: stable prefix");
        assert!(partial.consumed <= end, "RR-0203: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0203: full prefix should match bulk checksum");
        }
    }
}
