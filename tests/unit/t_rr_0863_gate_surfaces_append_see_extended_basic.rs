//! Integration test for `RR-0863` (basic).
//! Extended: Gate surfaces append seek refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0863_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x6a];
    let first = relayring::capabilities::rr_0863_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0863: Extended: Gate surfaces append seek refactor mutator v28");
    let second = relayring::capabilities::rr_0863_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0863: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0863: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0863: stats visits every byte");
}
