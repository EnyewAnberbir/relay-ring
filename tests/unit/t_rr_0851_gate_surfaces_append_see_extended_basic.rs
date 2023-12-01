//! Integration test for `RR-0851` (basic).
//! Extended: Gate surfaces append seek export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0851_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5c, 0x5e];
    let first = relayring::capabilities::rr_0851_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0851: Extended: Gate surfaces append seek export adapter v16");
    let second = relayring::capabilities::rr_0851_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0851: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0851: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0851: stats visits every byte");
}
