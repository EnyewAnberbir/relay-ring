//! Integration test for `RR-0371` (basic).
//! Gate surfaces seal index export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0371_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x7a];
    let first = relayring::capabilities::rr_0371_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0371: Gate surfaces seal index export adapter v6");
    let second = relayring::capabilities::rr_0371_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0371: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0371: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0371: scanner should emit domain hints");
}
