//! Integration test for `RR-0869` (basic).
//! Extended: Gate surfaces seal index optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0869_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x70];
    let first = relayring::capabilities::rr_0869_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0869: Extended: Gate surfaces seal index optimize registry v4");
    let second = relayring::capabilities::rr_0869_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0869: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0869: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0869: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
