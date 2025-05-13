//! Integration test for `RR-0681` (stability).
//! Extended: Journal append seal export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0681_journal_append_seal_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let full = relayring::capabilities::rr_0681_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0681: bulk Extended: Journal append seal export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0681_journal_append_seal_expo_extended::evaluate(&fixture[..end]).expect("RR-0681: stable prefix");
        assert!(partial.consumed <= end, "RR-0681: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0681: full prefix should match bulk checksum");
        }
    }
}
