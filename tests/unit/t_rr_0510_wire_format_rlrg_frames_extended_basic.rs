//! Integration test for `RR-0510` (basic).
//! Extended: Wire format RLRG frames implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0510_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let first = relayring::capabilities::rr_0510_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0510: Extended: Wire format RLRG frames implement pipeline v10");
    let second = relayring::capabilities::rr_0510_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0510: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0510: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0510: stats visits every byte");
}
