//! Integration test for `RR-0018` (roundtrip).
//! Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0018_wire_format_rlrg_frames_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let a = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0018 first pass");
    let b = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
