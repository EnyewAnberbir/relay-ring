//! Integration test for `RR-0350` (basic).
//! Gate surfaces append seek validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0350_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x65];
    let first = relayring::capabilities::rr_0350_gate_surfaces_append_see::evaluate(fixture).expect("RR-0350: Gate surfaces append seek validate resolver v15");
    let second = relayring::capabilities::rr_0350_gate_surfaces_append_see::evaluate(fixture).expect("RR-0350: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0350: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0350: scanner should emit domain hints");
}
