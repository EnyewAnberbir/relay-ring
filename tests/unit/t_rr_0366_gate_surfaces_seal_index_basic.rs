//! Integration test for `RR-0366` (basic).
//! Gate surfaces seal index extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0366_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x75];
    let first = relayring::capabilities::rr_0366_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0366: Gate surfaces seal index extend codec v1");
    let second = relayring::capabilities::rr_0366_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0366: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0366: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0366: stats visits every byte");
}
