//! Integration test for `RR-0381` (basic).
//! Gate surfaces seal index export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0381_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x84];
    let first = relayring::capabilities::rr_0381_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0381: Gate surfaces seal index export adapter v16");
    let second = relayring::capabilities::rr_0381_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0381: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0381: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0381: scanner should emit domain hints");
}
