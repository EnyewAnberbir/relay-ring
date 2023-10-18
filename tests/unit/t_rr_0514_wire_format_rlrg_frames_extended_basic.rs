//! Integration test for `RR-0514` (basic).
//! Extended: Wire format RLRG frames optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0514_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let first = relayring::capabilities::rr_0514_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0514: Extended: Wire format RLRG frames optimize registry v14");
    let second = relayring::capabilities::rr_0514_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0514: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0514: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0514: stats visits every byte");
}
