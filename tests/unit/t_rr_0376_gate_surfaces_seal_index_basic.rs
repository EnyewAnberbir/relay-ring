//! Integration test for `RR-0376` (basic).
//! Gate surfaces seal index extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0376_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x7f];
    let first = relayring::capabilities::rr_0376_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0376: Gate surfaces seal index extend codec v11");
    let second = relayring::capabilities::rr_0376_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0376: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0376: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0376: scanner should emit domain hints");
}
