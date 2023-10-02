//! Integration test for `RR-0389` (basic).
//! Gate surfaces seal index optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0389_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x8c];
    let first = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0389: Gate surfaces seal index optimize registry v24");
    let second = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0389: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0389: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0389: stats visits every byte");
}
