//! Integration test for `RR-0348` (basic).
//! Gate surfaces append seek wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0348_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x63];
    let first = relayring::capabilities::rr_0348_gate_surfaces_append_see::evaluate(fixture).expect("RR-0348: Gate surfaces append seek wire planner v13");
    let second = relayring::capabilities::rr_0348_gate_surfaces_append_see::evaluate(fixture).expect("RR-0348: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0348: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0348: window consumes the whole buffer");
}
