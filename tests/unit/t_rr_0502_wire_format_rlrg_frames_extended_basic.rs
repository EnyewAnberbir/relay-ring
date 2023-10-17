//! Integration test for `RR-0502` (basic).
//! Extended: Wire format RLRG frames harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0502_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let first = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0502: Extended: Wire format RLRG frames harden index v2");
    let second = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0502: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0502: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0502: stats visits every byte");
}
