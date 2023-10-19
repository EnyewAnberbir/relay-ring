//! Integration test for `RR-0525` (basic).
//! Extended: Wire format RLRG frames validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0525_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let first = relayring::capabilities::rr_0525_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0525: Extended: Wire format RLRG frames validate resolver v25");
    let second = relayring::capabilities::rr_0525_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0525: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0525: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0525: window consumes the whole buffer");
}
