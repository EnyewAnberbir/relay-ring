//! Integration test for `RR-0690` (stability).
//! Extended: Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0690_journal_append_seal_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let full = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0690: bulk Extended: Journal append seal validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(&fixture[..end]).expect("RR-0690: stable prefix");
        assert!(partial.consumed <= end, "RR-0690: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0690: full prefix should match bulk checksum");
        }
    }
}
