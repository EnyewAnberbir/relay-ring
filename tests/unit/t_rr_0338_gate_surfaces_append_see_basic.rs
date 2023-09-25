//! Integration test for `RR-0338` (basic).
//! Gate surfaces append seek wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0338_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x59];
    let first = relayring::capabilities::rr_0338_gate_surfaces_append_see::evaluate(fixture).expect("RR-0338: Gate surfaces append seek wire planner v3");
    let second = relayring::capabilities::rr_0338_gate_surfaces_append_see::evaluate(fixture).expect("RR-0338: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0338: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0338: scanner should emit domain hints");
}
