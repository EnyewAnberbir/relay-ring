//! Integration test for `RR-0184` (basic).
//! Journal append seal benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0184_journal_append_seal_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let first = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("RR-0184: Journal append seal benchmark reporter v9");
    let second = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("RR-0184: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0184: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0184: window consumes the whole buffer");
}
