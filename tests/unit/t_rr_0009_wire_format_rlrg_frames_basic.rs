//! Integration test for `RR-0009` (basic).
//! Wire format RLRG frames benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0009_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let first = relayring::capabilities::rr_0009_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0009: Wire format RLRG frames benchmark reporter v9");
    let second = relayring::capabilities::rr_0009_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0009: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0009: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0009: window consumes the whole buffer");
}
