//! Integration test for `RR-0894` (basic).
//! Extended: Gate surfaces seal index benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0894_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89];
    let first = relayring::capabilities::rr_0894_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0894: Extended: Gate surfaces seal index benchmark reporter v29");
    let second = relayring::capabilities::rr_0894_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0894: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0894: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0894: stats visits every byte");
}
