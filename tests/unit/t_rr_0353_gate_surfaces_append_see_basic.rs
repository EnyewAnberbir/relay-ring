//! Integration test for `RR-0353` (basic).
//! Gate surfaces append seek refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0353_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x68];
    let first = relayring::capabilities::rr_0353_gate_surfaces_append_see::evaluate(fixture).expect("RR-0353: Gate surfaces append seek refactor mutator v18");
    let second = relayring::capabilities::rr_0353_gate_surfaces_append_see::evaluate(fixture).expect("RR-0353: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0353: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0353: window consumes the whole buffer");
}
