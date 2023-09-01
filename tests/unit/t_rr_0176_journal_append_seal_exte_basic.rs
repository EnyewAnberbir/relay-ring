//! Integration test for `RR-0176` (basic).
//! Journal append seal extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0176_journal_append_seal_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let first = relayring::capabilities::rr_0176_journal_append_seal_exte::evaluate(fixture).expect("RR-0176: Journal append seal extend codec v1");
    let second = relayring::capabilities::rr_0176_journal_append_seal_exte::evaluate(fixture).expect("RR-0176: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0176: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0176: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
