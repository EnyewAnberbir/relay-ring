//! Integration test for `RR-0884` (basic).
//! Extended: Gate surfaces seal index benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0884_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x7f];
    let first = relayring::capabilities::rr_0884_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0884: Extended: Gate surfaces seal index benchmark reporter v19");
    let second = relayring::capabilities::rr_0884_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0884: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0884: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0884: window consumes the whole buffer");
}
