//! Integration test for `RR-0351` (basic).
//! Gate surfaces append seek export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0351_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x66];
    let first = relayring::capabilities::rr_0351_gate_surfaces_append_see::evaluate(fixture).expect("RR-0351: Gate surfaces append seek export adapter v16");
    let second = relayring::capabilities::rr_0351_gate_surfaces_append_see::evaluate(fixture).expect("RR-0351: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0351: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0351: stats visits every byte");
}
