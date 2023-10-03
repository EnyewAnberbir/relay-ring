//! Integration test for `RR-0393` (basic).
//! Gate surfaces seal index refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0393_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8e, 0x90];
    let first = relayring::capabilities::rr_0393_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0393: Gate surfaces seal index refactor mutator v28");
    let second = relayring::capabilities::rr_0393_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0393: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0393: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0393: window consumes the whole buffer");
}
