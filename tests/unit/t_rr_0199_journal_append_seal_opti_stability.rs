//! Integration test for `RR-0199` (stability).
//! Journal append seal optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0199_journal_append_seal_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xca, 0xcc];
    let full = relayring::capabilities::rr_0199_journal_append_seal_opti::evaluate(fixture).expect("RR-0199: bulk Journal append seal optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0199_journal_append_seal_opti::evaluate(&fixture[..end]).expect("RR-0199: stable prefix");
        assert!(partial.consumed <= end, "RR-0199: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0199: full prefix should match bulk checksum");
        }
    }
}
