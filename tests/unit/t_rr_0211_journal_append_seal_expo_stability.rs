//! Integration test for `RR-0211` (stability).
//! Journal append seal export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0211_journal_append_seal_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let full = relayring::capabilities::rr_0211_journal_append_seal_expo::evaluate(fixture).expect("RR-0211: bulk Journal append seal export adapter v36");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0211_journal_append_seal_expo::evaluate(&fixture[..end]).expect("RR-0211: stable prefix");
        assert!(partial.consumed <= end, "RR-0211: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0211: full prefix should match bulk checksum");
        }
    }
}
