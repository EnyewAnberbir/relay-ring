//! Integration test for `RR-0379` (basic).
//! Gate surfaces seal index optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0379_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let first = relayring::capabilities::rr_0379_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0379: Gate surfaces seal index optimize registry v14");
    let second = relayring::capabilities::rr_0379_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0379: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0379: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0379: scanner should emit domain hints");
}
