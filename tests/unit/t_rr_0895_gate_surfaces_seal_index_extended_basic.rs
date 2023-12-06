//! Integration test for `RR-0895` (basic).
//! Extended: Gate surfaces seal index implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0895_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let first = relayring::capabilities::rr_0895_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0895: Extended: Gate surfaces seal index implement pipeline v30");
    let second = relayring::capabilities::rr_0895_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0895: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0895: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0895: scanner should emit domain hints");
}
