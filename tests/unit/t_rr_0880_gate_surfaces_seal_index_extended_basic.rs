//! Integration test for `RR-0880` (basic).
//! Extended: Gate surfaces seal index validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0880_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79, 0x7b];
    let first = relayring::capabilities::rr_0880_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0880: Extended: Gate surfaces seal index validate resolver v15");
    let second = relayring::capabilities::rr_0880_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0880: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0880: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0880: scanner should emit domain hints");
}
