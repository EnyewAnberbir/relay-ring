//! Integration test for `RR-0181` (stability).
//! Journal append seal export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0181_journal_append_seal_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let full = relayring::capabilities::rr_0181_journal_append_seal_expo::evaluate(fixture).expect("RR-0181: bulk Journal append seal export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0181_journal_append_seal_expo::evaluate(&fixture[..end]).expect("RR-0181: stable prefix");
        assert!(partial.consumed <= end, "RR-0181: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0181: full prefix should match bulk checksum");
        }
    }
}
