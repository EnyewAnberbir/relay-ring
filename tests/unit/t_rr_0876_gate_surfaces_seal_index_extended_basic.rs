//! Integration test for `RR-0876` (basic).
//! Extended: Gate surfaces seal index extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0876_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x75, 0x77];
    let first = relayring::capabilities::rr_0876_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0876: Extended: Gate surfaces seal index extend codec v11");
    let second = relayring::capabilities::rr_0876_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0876: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0876: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0876: stats visits every byte");
}
