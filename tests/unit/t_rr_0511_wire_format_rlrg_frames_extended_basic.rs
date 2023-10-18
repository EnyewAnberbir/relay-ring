//! Integration test for `RR-0511` (basic).
//! Extended: Wire format RLRG frames extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0511_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let first = relayring::capabilities::rr_0511_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0511: Extended: Wire format RLRG frames extend codec v11");
    let second = relayring::capabilities::rr_0511_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0511: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0511: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0511: scanner should emit domain hints");
}
