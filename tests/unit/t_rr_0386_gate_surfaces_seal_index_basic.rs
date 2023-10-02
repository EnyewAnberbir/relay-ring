//! Integration test for `RR-0386` (basic).
//! Gate surfaces seal index extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0386_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89];
    let first = relayring::capabilities::rr_0386_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0386: Gate surfaces seal index extend codec v21");
    let second = relayring::capabilities::rr_0386_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0386: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0386: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0386: stats visits every byte");
}
