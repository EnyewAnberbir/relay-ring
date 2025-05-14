//! Integration test for `RR-0691` (stability).
//! Extended: Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0691_journal_append_seal_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let full = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0691: bulk Extended: Journal append seal export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(&fixture[..end]).expect("RR-0691: stable prefix");
        assert!(partial.consumed <= end, "RR-0691: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0691: full prefix should match bulk checksum");
        }
    }
}
