//! Integration test for `RR-0384` (basic).
//! Gate surfaces seal index benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0384_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let first = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0384: Gate surfaces seal index benchmark reporter v19");
    let second = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0384: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0384: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0384: stats visits every byte");
}
