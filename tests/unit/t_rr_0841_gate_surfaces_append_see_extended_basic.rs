//! Integration test for `RR-0841` (basic).
//! Extended: Gate surfaces append seek export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0841_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let first = relayring::capabilities::rr_0841_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0841: Extended: Gate surfaces append seek export adapter v6");
    let second = relayring::capabilities::rr_0841_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0841: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0841: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0841: stats visits every byte");
}
