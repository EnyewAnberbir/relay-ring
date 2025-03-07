//! Integration test for `RR-0212` (stability).
//! Journal append seal integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0212_journal_append_seal_inte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let full = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(fixture).expect("RR-0212: bulk Journal append seal integrate validator v37");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(&fixture[..end]).expect("RR-0212: stable prefix");
        assert!(partial.consumed <= end, "RR-0212: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0212: full prefix should match bulk checksum");
        }
    }
}
