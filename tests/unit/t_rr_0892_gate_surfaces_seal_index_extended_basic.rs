//! Integration test for `RR-0892` (basic).
//! Extended: Gate surfaces seal index integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0892_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let first = relayring::capabilities::rr_0892_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0892: Extended: Gate surfaces seal index integrate validator v27");
    let second = relayring::capabilities::rr_0892_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0892: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0892: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0892: scanner should emit domain hints");
}
