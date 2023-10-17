//! Integration test for `RR-0501` (basic).
//! Extended: Wire format RLRG frames extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0501_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let first = relayring::capabilities::rr_0501_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0501: Extended: Wire format RLRG frames extend codec v1");
    let second = relayring::capabilities::rr_0501_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0501: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0501: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0501: scanner should emit domain hints");
}
