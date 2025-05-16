//! Integration test for `RR-0706` (stability).
//! Extended: Journal append seal extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0706_journal_append_seal_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let full = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0706: bulk Extended: Journal append seal extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(&fixture[..end]).expect("RR-0706: stable prefix");
        assert!(partial.consumed <= end, "RR-0706: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0706: full prefix should match bulk checksum");
        }
    }
}
