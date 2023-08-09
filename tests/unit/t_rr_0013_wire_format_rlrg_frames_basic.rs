//! Integration test for `RR-0013` (basic).
//! Wire format RLRG frames wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0013_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let first = relayring::capabilities::rr_0013_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0013: Wire format RLRG frames wire planner v13");
    let second = relayring::capabilities::rr_0013_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0013: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0013: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0013: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
