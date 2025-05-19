//! Integration test for `RR-0710` (stability).
//! Extended: Journal append seal validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0710_journal_append_seal_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let full = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0710: bulk Extended: Journal append seal validate resolver v35");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(&fixture[..end]).expect("RR-0710: stable prefix");
        assert!(partial.consumed <= end, "RR-0710: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0710: full prefix should match bulk checksum");
        }
    }
}
