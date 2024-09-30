//! Integration test for `RR-0023` (roundtrip).
//! Wire format RLRG frames wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0023_wire_format_rlrg_frames_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let a = relayring::capabilities::rr_0023_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0023 first pass");
    let b = relayring::capabilities::rr_0023_wire_format_rlrg_frames::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
