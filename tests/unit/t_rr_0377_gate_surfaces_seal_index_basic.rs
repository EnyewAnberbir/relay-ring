//! Integration test for `RR-0377` (basic).
//! Gate surfaces seal index harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0377_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x80];
    let first = relayring::capabilities::rr_0377_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0377: Gate surfaces seal index harden index v12");
    let second = relayring::capabilities::rr_0377_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0377: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0377: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0377: scanner should emit domain hints");
}
