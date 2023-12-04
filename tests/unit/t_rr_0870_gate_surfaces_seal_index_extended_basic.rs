//! Integration test for `RR-0870` (basic).
//! Extended: Gate surfaces seal index validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0870_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let first = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0870: Extended: Gate surfaces seal index validate resolver v5");
    let second = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0870: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0870: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0870: window consumes the whole buffer");
}
