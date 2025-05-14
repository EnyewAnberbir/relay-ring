//! Integration test for `RR-0689` (stability).
//! Extended: Journal append seal optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0689_journal_append_seal_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let full = relayring::capabilities::rr_0689_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0689: bulk Extended: Journal append seal optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0689_journal_append_seal_opti_extended::evaluate(&fixture[..end]).expect("RR-0689: stable prefix");
        assert!(partial.consumed <= end, "RR-0689: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0689: full prefix should match bulk checksum");
        }
    }
}
