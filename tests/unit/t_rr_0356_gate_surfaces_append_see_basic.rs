//! Integration test for `RR-0356` (basic).
//! Gate surfaces append seek extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0356_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let first = relayring::capabilities::rr_0356_gate_surfaces_append_see::evaluate(fixture).expect("RR-0356: Gate surfaces append seek extend codec v21");
    let second = relayring::capabilities::rr_0356_gate_surfaces_append_see::evaluate(fixture).expect("RR-0356: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0356: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0356: scanner should emit domain hints");
}
