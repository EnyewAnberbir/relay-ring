//! Integration test for `RR-0375` (basic).
//! Gate surfaces seal index implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0375_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let first = relayring::capabilities::rr_0375_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0375: Gate surfaces seal index implement pipeline v10");
    let second = relayring::capabilities::rr_0375_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0375: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0375: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0375: window consumes the whole buffer");
}
