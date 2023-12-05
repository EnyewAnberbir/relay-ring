//! Integration test for `RR-0886` (basic).
//! Extended: Gate surfaces seal index extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0886_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let first = relayring::capabilities::rr_0886_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0886: Extended: Gate surfaces seal index extend codec v21");
    let second = relayring::capabilities::rr_0886_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0886: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0886: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0886: stats visits every byte");
}
