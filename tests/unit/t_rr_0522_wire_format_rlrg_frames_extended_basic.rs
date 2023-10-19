//! Integration test for `RR-0522` (basic).
//! Extended: Wire format RLRG frames harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0522_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let first = relayring::capabilities::rr_0522_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0522: Extended: Wire format RLRG frames harden index v22");
    let second = relayring::capabilities::rr_0522_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0522: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0522: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0522: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
