//! Integration test for `RR-0354` (basic).
//! Gate surfaces append seek benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0354_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x69];
    let first = relayring::capabilities::rr_0354_gate_surfaces_append_see::evaluate(fixture).expect("RR-0354: Gate surfaces append seek benchmark reporter v19");
    let second = relayring::capabilities::rr_0354_gate_surfaces_append_see::evaluate(fixture).expect("RR-0354: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0354: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0354: scanner should emit domain hints");
}
