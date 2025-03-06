//! Integration test for `RR-0202` (stability).
//! Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0202_journal_append_seal_inte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let full = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("RR-0202: bulk Journal append seal integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(&fixture[..end]).expect("RR-0202: stable prefix");
        assert!(partial.consumed <= end, "RR-0202: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0202: full prefix should match bulk checksum");
        }
    }
}
