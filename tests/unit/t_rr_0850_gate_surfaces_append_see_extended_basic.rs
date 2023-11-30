//! Integration test for `RR-0850` (basic).
//! Extended: Gate surfaces append seek validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0850_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x5d];
    let first = relayring::capabilities::rr_0850_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0850: Extended: Gate surfaces append seek validate resolver v15");
    let second = relayring::capabilities::rr_0850_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0850: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0850: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0850: window consumes the whole buffer");
}
