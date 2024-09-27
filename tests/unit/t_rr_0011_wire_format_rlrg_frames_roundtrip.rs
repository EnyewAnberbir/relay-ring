//! Integration test for `RR-0011` (roundtrip).
//! Wire format RLRG frames extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0011_wire_format_rlrg_frames_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let a = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0011 first pass");
    let b = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
