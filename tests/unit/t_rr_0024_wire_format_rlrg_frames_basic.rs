//! Integration test for `RR-0024` (basic).
//! Wire format RLRG frames optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0024_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let first = relayring::capabilities::rr_0024_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0024: Wire format RLRG frames optimize registry v24");
    let second = relayring::capabilities::rr_0024_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0024: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0024: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0024: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
