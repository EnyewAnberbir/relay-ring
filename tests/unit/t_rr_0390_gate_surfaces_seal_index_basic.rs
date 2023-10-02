//! Integration test for `RR-0390` (basic).
//! Gate surfaces seal index validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0390_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let first = relayring::capabilities::rr_0390_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0390: Gate surfaces seal index validate resolver v25");
    let second = relayring::capabilities::rr_0390_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0390: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0390: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0390: scanner should emit domain hints");
}
