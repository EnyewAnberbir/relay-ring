//! Integration test for `RR-0361` (basic).
//! Gate surfaces append seek export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0361_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x70];
    let first = relayring::capabilities::rr_0361_gate_surfaces_append_see::evaluate(fixture).expect("RR-0361: Gate surfaces append seek export adapter v26");
    let second = relayring::capabilities::rr_0361_gate_surfaces_append_see::evaluate(fixture).expect("RR-0361: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0361: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0361: window consumes the whole buffer");
}
