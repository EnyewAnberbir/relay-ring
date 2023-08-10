//! Integration test for `RR-0021` (basic).
//! Wire format RLRG frames extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0021_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let first = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0021: Wire format RLRG frames extend codec v21");
    let second = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0021: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0021: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0021: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
