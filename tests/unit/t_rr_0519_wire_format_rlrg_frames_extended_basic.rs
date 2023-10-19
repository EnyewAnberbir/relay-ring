//! Integration test for `RR-0519` (basic).
//! Extended: Wire format RLRG frames benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0519_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let first = relayring::capabilities::rr_0519_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0519: Extended: Wire format RLRG frames benchmark reporter v19");
    let second = relayring::capabilities::rr_0519_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0519: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0519: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0519: scanner should emit domain hints");
}
