//! Integration test for `RR-0200` (basic).
//! Journal append seal validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0200_journal_append_seal_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let first = relayring::capabilities::rr_0200_journal_append_seal_vali::evaluate(fixture).expect("RR-0200: Journal append seal validate resolver v25");
    let second = relayring::capabilities::rr_0200_journal_append_seal_vali::evaluate(fixture).expect("RR-0200: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0200: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0200: scanner should emit domain hints");
}
