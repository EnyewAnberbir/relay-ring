//! Integration test for `RR-0395` (basic).
//! Gate surfaces seal index implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0395_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let first = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0395: Gate surfaces seal index implement pipeline v30");
    let second = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0395: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0395: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0395: scanner should emit domain hints");
}
