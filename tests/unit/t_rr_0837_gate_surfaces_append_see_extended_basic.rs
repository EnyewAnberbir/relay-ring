//! Integration test for `RR-0837` (basic).
//! Extended: Gate surfaces append seek harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0837_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let first = relayring::capabilities::rr_0837_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0837: Extended: Gate surfaces append seek harden index v2");
    let second = relayring::capabilities::rr_0837_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0837: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0837: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0837: scanner should emit domain hints");
}
