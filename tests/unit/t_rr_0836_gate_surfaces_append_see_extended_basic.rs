//! Integration test for `RR-0836` (basic).
//! Extended: Gate surfaces append seek extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0836_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let first = relayring::capabilities::rr_0836_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0836: Extended: Gate surfaces append seek extend codec v1");
    let second = relayring::capabilities::rr_0836_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0836: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0836: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0836: scanner should emit domain hints");
}
