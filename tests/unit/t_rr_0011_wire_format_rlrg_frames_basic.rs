//! Integration test for `RR-0011` (basic).
//! Wire format RLRG frames extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0011_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let first = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0011: Wire format RLRG frames extend codec v11");
    let second = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0011: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0011: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0011: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
