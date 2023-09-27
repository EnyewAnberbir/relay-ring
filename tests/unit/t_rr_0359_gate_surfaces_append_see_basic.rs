//! Integration test for `RR-0359` (basic).
//! Gate surfaces append seek optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0359_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let first = relayring::capabilities::rr_0359_gate_surfaces_append_see::evaluate(fixture).expect("RR-0359: Gate surfaces append seek optimize registry v24");
    let second = relayring::capabilities::rr_0359_gate_surfaces_append_see::evaluate(fixture).expect("RR-0359: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0359: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0359: window consumes the whole buffer");
}
