//! Integration test for `RR-0883` (basic).
//! Extended: Gate surfaces seal index refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0883_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let first = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0883: Extended: Gate surfaces seal index refactor mutator v18");
    let second = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0883: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0883: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0883: stats visits every byte");
}
