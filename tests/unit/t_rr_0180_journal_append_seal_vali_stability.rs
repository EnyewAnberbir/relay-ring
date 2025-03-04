//! Integration test for `RR-0180` (stability).
//! Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0180_journal_append_seal_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let full = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180: bulk Journal append seal validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(&fixture[..end]).expect("RR-0180: stable prefix");
        assert!(partial.consumed <= end, "RR-0180: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0180: full prefix should match bulk checksum");
        }
    }
}
