//! Integration test for `RR-0002` (basic).
//! Wire format RLRG frames harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0002_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let first = relayring::capabilities::rr_0002_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0002: Wire format RLRG frames harden index v2");
    let second = relayring::capabilities::rr_0002_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0002: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0002: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0002: stats visits every byte");
}
