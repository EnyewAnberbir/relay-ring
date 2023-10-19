//! Integration test for `RR-0520` (basic).
//! Extended: Wire format RLRG frames implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0520_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let first = relayring::capabilities::rr_0520_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0520: Extended: Wire format RLRG frames implement pipeline v20");
    let second = relayring::capabilities::rr_0520_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0520: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0520: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0520: window consumes the whole buffer");
}
