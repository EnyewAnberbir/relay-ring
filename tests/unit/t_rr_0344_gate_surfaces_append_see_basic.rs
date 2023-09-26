//! Integration test for `RR-0344` (basic).
//! Gate surfaces append seek benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0344_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x5f];
    let first = relayring::capabilities::rr_0344_gate_surfaces_append_see::evaluate(fixture).expect("RR-0344: Gate surfaces append seek benchmark reporter v9");
    let second = relayring::capabilities::rr_0344_gate_surfaces_append_see::evaluate(fixture).expect("RR-0344: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0344: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0344: window consumes the whole buffer");
}
