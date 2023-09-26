//! Integration test for `RR-0347` (basic).
//! Gate surfaces append seek harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0347_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x62];
    let first = relayring::capabilities::rr_0347_gate_surfaces_append_see::evaluate(fixture).expect("RR-0347: Gate surfaces append seek harden index v12");
    let second = relayring::capabilities::rr_0347_gate_surfaces_append_see::evaluate(fixture).expect("RR-0347: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0347: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0347: scanner should emit domain hints");
}
