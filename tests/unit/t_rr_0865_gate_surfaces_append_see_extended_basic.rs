//! Integration test for `RR-0865` (basic).
//! Extended: Gate surfaces append seek implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0865_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6a, 0x6c];
    let first = relayring::capabilities::rr_0865_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0865: Extended: Gate surfaces append seek implement pipeline v30");
    let second = relayring::capabilities::rr_0865_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0865: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0865: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0865: scanner should emit domain hints");
}
