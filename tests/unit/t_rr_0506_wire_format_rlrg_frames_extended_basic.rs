//! Integration test for `RR-0506` (basic).
//! Extended: Wire format RLRG frames export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0506_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let first = relayring::capabilities::rr_0506_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0506: Extended: Wire format RLRG frames export adapter v6");
    let second = relayring::capabilities::rr_0506_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0506: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0506: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0506: window consumes the whole buffer");
}
