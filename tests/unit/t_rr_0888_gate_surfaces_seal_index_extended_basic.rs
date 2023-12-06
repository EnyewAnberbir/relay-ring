//! Integration test for `RR-0888` (basic).
//! Extended: Gate surfaces seal index wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0888_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81, 0x83];
    let first = relayring::capabilities::rr_0888_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0888: Extended: Gate surfaces seal index wire planner v23");
    let second = relayring::capabilities::rr_0888_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0888: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0888: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0888: scanner should emit domain hints");
}
