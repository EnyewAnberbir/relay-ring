//! Integration test for `RR-0012` (basic).
//! Wire format RLRG frames harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0012_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let first = relayring::capabilities::rr_0012_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0012: Wire format RLRG frames harden index v12");
    let second = relayring::capabilities::rr_0012_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0012: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0012: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0012: stats visits every byte");
}
