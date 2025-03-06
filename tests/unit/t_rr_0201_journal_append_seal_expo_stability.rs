//! Integration test for `RR-0201` (stability).
//! Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0201_journal_append_seal_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let full = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(fixture).expect("RR-0201: bulk Journal append seal export adapter v26");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(&fixture[..end]).expect("RR-0201: stable prefix");
        assert!(partial.consumed <= end, "RR-0201: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0201: full prefix should match bulk checksum");
        }
    }
}
