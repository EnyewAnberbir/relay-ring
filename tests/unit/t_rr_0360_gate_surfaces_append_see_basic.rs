//! Integration test for `RR-0360` (basic).
//! Gate surfaces append seek validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0360_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x6f];
    let first = relayring::capabilities::rr_0360_gate_surfaces_append_see::evaluate(fixture).expect("RR-0360: Gate surfaces append seek validate resolver v25");
    let second = relayring::capabilities::rr_0360_gate_surfaces_append_see::evaluate(fixture).expect("RR-0360: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0360: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0360: window consumes the whole buffer");
}
