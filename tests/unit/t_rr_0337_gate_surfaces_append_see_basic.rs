//! Integration test for `RR-0337` (basic).
//! Gate surfaces append seek harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0337_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x58];
    let first = relayring::capabilities::rr_0337_gate_surfaces_append_see::evaluate(fixture).expect("RR-0337: Gate surfaces append seek harden index v2");
    let second = relayring::capabilities::rr_0337_gate_surfaces_append_see::evaluate(fixture).expect("RR-0337: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0337: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0337: scanner should emit domain hints");
}
