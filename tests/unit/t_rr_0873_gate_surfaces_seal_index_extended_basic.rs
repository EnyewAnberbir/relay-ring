//! Integration test for `RR-0873` (basic).
//! Extended: Gate surfaces seal index refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0873_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x72, 0x74];
    let first = relayring::capabilities::rr_0873_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0873: Extended: Gate surfaces seal index refactor mutator v8");
    let second = relayring::capabilities::rr_0873_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0873: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0873: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0873: stats visits every byte");
}
