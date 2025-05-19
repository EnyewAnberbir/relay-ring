//! Integration test for `RR-0714` (stability).
//! Extended: Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0714_journal_append_seal_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let full = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0714: bulk Extended: Journal append seal benchmark reporter v39");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(&fixture[..end]).expect("RR-0714: stable prefix");
        assert!(partial.consumed <= end, "RR-0714: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0714: full prefix should match bulk checksum");
        }
    }
}
