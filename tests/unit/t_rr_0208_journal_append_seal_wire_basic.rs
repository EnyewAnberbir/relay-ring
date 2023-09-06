//! Integration test for `RR-0208` (basic).
//! Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0208_journal_append_seal_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let first = relayring::capabilities::rr_0208_journal_append_seal_wire::evaluate(fixture).expect("RR-0208: Journal append seal wire planner v33");
    let second = relayring::capabilities::rr_0208_journal_append_seal_wire::evaluate(fixture).expect("RR-0208: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0208: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0208: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
