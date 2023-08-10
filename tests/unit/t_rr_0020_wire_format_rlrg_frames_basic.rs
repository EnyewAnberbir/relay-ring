//! Integration test for `RR-0020` (basic).
//! Wire format RLRG frames implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0020_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x19];
    let first = relayring::capabilities::rr_0020_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0020: Wire format RLRG frames implement pipeline v20");
    let second = relayring::capabilities::rr_0020_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0020: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0020: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0020: window consumes the whole buffer");
}
