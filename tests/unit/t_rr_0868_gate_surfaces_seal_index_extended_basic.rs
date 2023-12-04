//! Integration test for `RR-0868` (basic).
//! Extended: Gate surfaces seal index wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0868_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x6f];
    let first = relayring::capabilities::rr_0868_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0868: Extended: Gate surfaces seal index wire planner v3");
    let second = relayring::capabilities::rr_0868_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0868: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0868: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0868: scanner should emit domain hints");
}
