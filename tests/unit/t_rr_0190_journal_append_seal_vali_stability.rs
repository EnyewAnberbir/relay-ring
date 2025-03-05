//! Integration test for `RR-0190` (stability).
//! Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0190_journal_append_seal_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let full = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("RR-0190: bulk Journal append seal validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(&fixture[..end]).expect("RR-0190: stable prefix");
        assert!(partial.consumed <= end, "RR-0190: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0190: full prefix should match bulk checksum");
        }
    }
}
