//! Integration test for `RR-0687` (stability).
//! Extended: Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0687_journal_append_seal_hard_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let full = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0687: bulk Extended: Journal append seal harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(&fixture[..end]).expect("RR-0687: stable prefix");
        assert!(partial.consumed <= end, "RR-0687: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0687: full prefix should match bulk checksum");
        }
    }
}
