//! Integration test for `RR-0524` (roundtrip).
//! Extended: Wire format RLRG frames optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0524_wire_format_rlrg_frames_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let a = relayring::capabilities::rr_0524_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0524 first pass");
    let b = relayring::capabilities::rr_0524_wire_format_rlrg_frames_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
