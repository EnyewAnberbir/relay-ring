//! Integration test for `RR-0189` (stability).
//! Journal append seal optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0189_journal_append_seal_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc2];
    let full = relayring::capabilities::rr_0189_journal_append_seal_opti::evaluate(fixture).expect("RR-0189: bulk Journal append seal optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0189_journal_append_seal_opti::evaluate(&fixture[..end]).expect("RR-0189: stable prefix");
        assert!(partial.consumed <= end, "RR-0189: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0189: full prefix should match bulk checksum");
        }
    }
}
