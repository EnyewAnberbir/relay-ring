//! Integration test for `RR-0358` (basic).
//! Gate surfaces append seek wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0358_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let first = relayring::capabilities::rr_0358_gate_surfaces_append_see::evaluate(fixture).expect("RR-0358: Gate surfaces append seek wire planner v23");
    let second = relayring::capabilities::rr_0358_gate_surfaces_append_see::evaluate(fixture).expect("RR-0358: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0358: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0358: scanner should emit domain hints");
}
