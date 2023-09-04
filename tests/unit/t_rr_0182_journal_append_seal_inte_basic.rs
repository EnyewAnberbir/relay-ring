//! Integration test for `RR-0182` (basic).
//! Journal append seal integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0182_journal_append_seal_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let first = relayring::capabilities::rr_0182_journal_append_seal_inte::evaluate(fixture).expect("RR-0182: Journal append seal integrate validator v7");
    let second = relayring::capabilities::rr_0182_journal_append_seal_inte::evaluate(fixture).expect("RR-0182: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0182: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0182: window consumes the whole buffer");
}
