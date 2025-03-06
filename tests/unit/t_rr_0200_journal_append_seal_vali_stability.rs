//! Integration test for `RR-0200` (stability).
//! Journal append seal validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0200_journal_append_seal_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let full = relayring::capabilities::rr_0200_journal_append_seal_vali::evaluate(fixture).expect("RR-0200: bulk Journal append seal validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0200_journal_append_seal_vali::evaluate(&fixture[..end]).expect("RR-0200: stable prefix");
        assert!(partial.consumed <= end, "RR-0200: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0200: full prefix should match bulk checksum");
        }
    }
}
