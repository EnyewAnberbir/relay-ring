//! Integration test for `RR-0391` (basic).
//! Gate surfaces seal index export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0391_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x8e];
    let first = relayring::capabilities::rr_0391_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0391: Gate surfaces seal index export adapter v26");
    let second = relayring::capabilities::rr_0391_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0391: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0391: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0391: stats visits every byte");
}
