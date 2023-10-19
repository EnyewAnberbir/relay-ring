//! Integration test for `RR-0524` (basic).
//! Extended: Wire format RLRG frames optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0524_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let first = relayring::capabilities::rr_0524_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0524: Extended: Wire format RLRG frames optimize registry v24");
    let second = relayring::capabilities::rr_0524_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0524: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0524: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0524: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
