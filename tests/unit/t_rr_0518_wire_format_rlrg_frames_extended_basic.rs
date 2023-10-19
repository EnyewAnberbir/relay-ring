//! Integration test for `RR-0518` (basic).
//! Extended: Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0518_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let first = relayring::capabilities::rr_0518_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0518: Extended: Wire format RLRG frames refactor mutator v18");
    let second = relayring::capabilities::rr_0518_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0518: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0518: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0518: stats visits every byte");
}
