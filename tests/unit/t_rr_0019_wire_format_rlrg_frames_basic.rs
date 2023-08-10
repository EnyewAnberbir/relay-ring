//! Integration test for `RR-0019` (basic).
//! Wire format RLRG frames benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0019_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let first = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0019: Wire format RLRG frames benchmark reporter v19");
    let second = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0019: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0019: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0019: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
