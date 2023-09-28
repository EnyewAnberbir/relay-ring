//! Integration test for `RR-0362` (basic).
//! Gate surfaces append seek integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0362_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let first = relayring::capabilities::rr_0362_gate_surfaces_append_see::evaluate(fixture).expect("RR-0362: Gate surfaces append seek integrate validator v27");
    let second = relayring::capabilities::rr_0362_gate_surfaces_append_see::evaluate(fixture).expect("RR-0362: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0362: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0362: window consumes the whole buffer");
}
