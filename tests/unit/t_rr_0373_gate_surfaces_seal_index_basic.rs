//! Integration test for `RR-0373` (basic).
//! Gate surfaces seal index refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0373_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x7c];
    let first = relayring::capabilities::rr_0373_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0373: Gate surfaces seal index refactor mutator v8");
    let second = relayring::capabilities::rr_0373_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0373: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0373: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0373: scanner should emit domain hints");
}
