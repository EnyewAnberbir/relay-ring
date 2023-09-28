//! Integration test for `RR-0367` (basic).
//! Gate surfaces seal index harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0367_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x76];
    let first = relayring::capabilities::rr_0367_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0367: Gate surfaces seal index harden index v2");
    let second = relayring::capabilities::rr_0367_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0367: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0367: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0367: stats visits every byte");
}
