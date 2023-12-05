//! Integration test for `RR-0879` (basic).
//! Extended: Gate surfaces seal index optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0879_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x7a];
    let first = relayring::capabilities::rr_0879_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0879: Extended: Gate surfaces seal index optimize registry v14");
    let second = relayring::capabilities::rr_0879_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0879: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0879: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0879: scanner should emit domain hints");
}
