//! Integration test for `RR-0021` (roundtrip).
//! Wire format RLRG frames extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0021_wire_format_rlrg_frames_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let a = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0021 first pass");
    let b = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
