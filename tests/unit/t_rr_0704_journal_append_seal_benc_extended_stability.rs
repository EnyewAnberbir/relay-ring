//! Integration test for `RR-0704` (stability).
//! Extended: Journal append seal benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0704_journal_append_seal_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let full = relayring::capabilities::rr_0704_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0704: bulk Extended: Journal append seal benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0704_journal_append_seal_benc_extended::evaluate(&fixture[..end]).expect("RR-0704: stable prefix");
        assert!(partial.consumed <= end, "RR-0704: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0704: full prefix should match bulk checksum");
        }
    }
}
