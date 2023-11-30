//! Integration test for `RR-0847` (basic).
//! Extended: Gate surfaces append seek harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0847_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let first = relayring::capabilities::rr_0847_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0847: Extended: Gate surfaces append seek harden index v12");
    let second = relayring::capabilities::rr_0847_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0847: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0847: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0847: window consumes the whole buffer");
}
