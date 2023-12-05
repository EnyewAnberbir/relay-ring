//! Integration test for `RR-0887` (basic).
//! Extended: Gate surfaces seal index harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0887_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let first = relayring::capabilities::rr_0887_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0887: Extended: Gate surfaces seal index harden index v22");
    let second = relayring::capabilities::rr_0887_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0887: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0887: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0887: window consumes the whole buffer");
}
