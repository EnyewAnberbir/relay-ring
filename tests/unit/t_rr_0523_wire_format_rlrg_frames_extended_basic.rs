//! Integration test for `RR-0523` (basic).
//! Extended: Wire format RLRG frames wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0523_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let first = relayring::capabilities::rr_0523_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0523: Extended: Wire format RLRG frames wire planner v23");
    let second = relayring::capabilities::rr_0523_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0523: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0523: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0523: scanner should emit domain hints");
}
