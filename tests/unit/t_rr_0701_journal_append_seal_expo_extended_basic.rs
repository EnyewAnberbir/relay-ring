//! Integration test for `RR-0701` (basic).
//! Extended: Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0701_journal_append_seal_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let first = relayring::capabilities::rr_0701_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0701: Extended: Journal append seal export adapter v26");
    let second = relayring::capabilities::rr_0701_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0701: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0701: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0701: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
