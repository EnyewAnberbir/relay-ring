//! Integration test for `RR-0023` (basic).
//! Wire format RLRG frames wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0023_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let first = relayring::capabilities::rr_0023_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0023: Wire format RLRG frames wire planner v23");
    let second = relayring::capabilities::rr_0023_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0023: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0023: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0023: stats visits every byte");
}
