//! Integration test for `RR-0867` (basic).
//! Extended: Gate surfaces seal index harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0867_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let first = relayring::capabilities::rr_0867_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0867: Extended: Gate surfaces seal index harden index v2");
    let second = relayring::capabilities::rr_0867_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0867: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0867: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0867: scanner should emit domain hints");
}
