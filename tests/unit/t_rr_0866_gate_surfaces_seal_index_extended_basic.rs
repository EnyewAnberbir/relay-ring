//! Integration test for `RR-0866` (basic).
//! Extended: Gate surfaces seal index extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0866_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let first = relayring::capabilities::rr_0866_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0866: Extended: Gate surfaces seal index extend codec v1");
    let second = relayring::capabilities::rr_0866_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0866: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0866: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0866: scanner should emit domain hints");
}
