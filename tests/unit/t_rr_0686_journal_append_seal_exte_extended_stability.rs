//! Integration test for `RR-0686` (stability).
//! Extended: Journal append seal extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0686_journal_append_seal_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let full = relayring::capabilities::rr_0686_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0686: bulk Extended: Journal append seal extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0686_journal_append_seal_exte_extended::evaluate(&fixture[..end]).expect("RR-0686: stable prefix");
        assert!(partial.consumed <= end, "RR-0686: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0686: full prefix should match bulk checksum");
        }
    }
}
