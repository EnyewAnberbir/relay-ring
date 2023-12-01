//! Integration test for `RR-0861` (basic).
//! Extended: Gate surfaces append seek export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0861_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x68];
    let first = relayring::capabilities::rr_0861_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0861: Extended: Gate surfaces append seek export adapter v26");
    let second = relayring::capabilities::rr_0861_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0861: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0861: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0861: window consumes the whole buffer");
}
