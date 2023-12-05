//! Integration test for `RR-0874` (basic).
//! Extended: Gate surfaces seal index benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0874_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x75];
    let first = relayring::capabilities::rr_0874_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0874: Extended: Gate surfaces seal index benchmark reporter v9");
    let second = relayring::capabilities::rr_0874_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0874: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0874: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0874: window consumes the whole buffer");
}
