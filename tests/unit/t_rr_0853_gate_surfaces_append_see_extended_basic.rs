//! Integration test for `RR-0853` (basic).
//! Extended: Gate surfaces append seek refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0853_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e, 0x60];
    let first = relayring::capabilities::rr_0853_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0853: Extended: Gate surfaces append seek refactor mutator v18");
    let second = relayring::capabilities::rr_0853_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0853: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0853: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0853: scanner should emit domain hints");
}
