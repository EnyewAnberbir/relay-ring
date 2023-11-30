//! Integration test for `RR-0848` (basic).
//! Extended: Gate surfaces append seek wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0848_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x5b];
    let first = relayring::capabilities::rr_0848_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0848: Extended: Gate surfaces append seek wire planner v13");
    let second = relayring::capabilities::rr_0848_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0848: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0848: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0848: stats visits every byte");
}
