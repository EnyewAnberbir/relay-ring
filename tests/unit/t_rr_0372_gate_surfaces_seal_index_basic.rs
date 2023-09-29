//! Integration test for `RR-0372` (basic).
//! Gate surfaces seal index integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0372_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79, 0x7b];
    let first = relayring::capabilities::rr_0372_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0372: Gate surfaces seal index integrate validator v7");
    let second = relayring::capabilities::rr_0372_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0372: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0372: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0372: stats visits every byte");
}
