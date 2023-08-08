//! Integration test for `RR-0003` (basic).
//! Wire format RLRG frames wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0003_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let first = relayring::capabilities::rr_0003_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0003: Wire format RLRG frames wire planner v3");
    let second = relayring::capabilities::rr_0003_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0003: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0003: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0003: scanner should emit domain hints");
}
