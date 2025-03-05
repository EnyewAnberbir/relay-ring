//! Integration test for `RR-0197` (stability).
//! Journal append seal harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0197_journal_append_seal_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let full = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("RR-0197: bulk Journal append seal harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(&fixture[..end]).expect("RR-0197: stable prefix");
        assert!(partial.consumed <= end, "RR-0197: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0197: full prefix should match bulk checksum");
        }
    }
}
