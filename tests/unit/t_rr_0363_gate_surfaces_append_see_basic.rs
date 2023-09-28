//! Integration test for `RR-0363` (basic).
//! Gate surfaces append seek refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0363_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x72];
    let first = relayring::capabilities::rr_0363_gate_surfaces_append_see::evaluate(fixture).expect("RR-0363: Gate surfaces append seek refactor mutator v28");
    let second = relayring::capabilities::rr_0363_gate_surfaces_append_see::evaluate(fixture).expect("RR-0363: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0363: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0363: stats visits every byte");
}
