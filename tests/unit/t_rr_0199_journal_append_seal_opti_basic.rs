//! Integration test for `RR-0199` (basic).
//! Journal append seal optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0199_journal_append_seal_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xca, 0xcc];
    let first = relayring::capabilities::rr_0199_journal_append_seal_opti::evaluate(fixture).expect("RR-0199: Journal append seal optimize registry v24");
    let second = relayring::capabilities::rr_0199_journal_append_seal_opti::evaluate(fixture).expect("RR-0199: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0199: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0199: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
