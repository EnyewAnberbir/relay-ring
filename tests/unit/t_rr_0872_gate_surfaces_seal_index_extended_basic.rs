//! Integration test for `RR-0872` (basic).
//! Extended: Gate surfaces seal index integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0872_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x71, 0x73];
    let first = relayring::capabilities::rr_0872_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0872: Extended: Gate surfaces seal index integrate validator v7");
    let second = relayring::capabilities::rr_0872_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0872: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0872: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0872: stats visits every byte");
}
