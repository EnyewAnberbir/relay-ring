//! Integration test for `RR-0380` (basic).
//! Gate surfaces seal index validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0380_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81, 0x83];
    let first = relayring::capabilities::rr_0380_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0380: Gate surfaces seal index validate resolver v15");
    let second = relayring::capabilities::rr_0380_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0380: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0380: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0380: stats visits every byte");
}
