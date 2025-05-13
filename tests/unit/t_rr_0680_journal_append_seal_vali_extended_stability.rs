//! Integration test for `RR-0680` (stability).
//! Extended: Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0680_journal_append_seal_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let full = relayring::capabilities::rr_0680_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0680: bulk Extended: Journal append seal validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0680_journal_append_seal_vali_extended::evaluate(&fixture[..end]).expect("RR-0680: stable prefix");
        assert!(partial.consumed <= end, "RR-0680: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0680: full prefix should match bulk checksum");
        }
    }
}
