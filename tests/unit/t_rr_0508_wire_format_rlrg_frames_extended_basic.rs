//! Integration test for `RR-0508` (basic).
//! Extended: Wire format RLRG frames refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0508_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let first = relayring::capabilities::rr_0508_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0508: Extended: Wire format RLRG frames refactor mutator v8");
    let second = relayring::capabilities::rr_0508_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0508: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0508: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0508: scanner should emit domain hints");
}
