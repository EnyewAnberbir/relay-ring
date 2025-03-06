//! Integration test for `RR-0207` (stability).
//! Journal append seal harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0207_journal_append_seal_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let full = relayring::capabilities::rr_0207_journal_append_seal_hard::evaluate(fixture).expect("RR-0207: bulk Journal append seal harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0207_journal_append_seal_hard::evaluate(&fixture[..end]).expect("RR-0207: stable prefix");
        assert!(partial.consumed <= end, "RR-0207: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0207: full prefix should match bulk checksum");
        }
    }
}
