//! Integration test for `RR-0504` (basic).
//! Extended: Wire format RLRG frames optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0504_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let first = relayring::capabilities::rr_0504_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0504: Extended: Wire format RLRG frames optimize registry v4");
    let second = relayring::capabilities::rr_0504_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0504: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0504: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0504: stats visits every byte");
}
