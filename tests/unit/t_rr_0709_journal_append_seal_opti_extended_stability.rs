//! Integration test for `RR-0709` (stability).
//! Extended: Journal append seal optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0709_journal_append_seal_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let full = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0709: bulk Extended: Journal append seal optimize registry v34");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(&fixture[..end]).expect("RR-0709: stable prefix");
        assert!(partial.consumed <= end, "RR-0709: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0709: full prefix should match bulk checksum");
        }
    }
}
