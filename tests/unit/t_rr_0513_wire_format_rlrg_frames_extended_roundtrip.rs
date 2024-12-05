//! Integration test for `RR-0513` (roundtrip).
//! Extended: Wire format RLRG frames wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0513_wire_format_rlrg_frames_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let a = relayring::capabilities::rr_0513_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0513 first pass");
    let b = relayring::capabilities::rr_0513_wire_format_rlrg_frames_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
