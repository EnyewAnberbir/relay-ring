//! Integration test for `RR-0004` (basic).
//! Wire format RLRG frames optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0004_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x09];
    let first = relayring::capabilities::rr_0004_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0004: Wire format RLRG frames optimize registry v4");
    let second = relayring::capabilities::rr_0004_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0004: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0004: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0004: stats visits every byte");
}
