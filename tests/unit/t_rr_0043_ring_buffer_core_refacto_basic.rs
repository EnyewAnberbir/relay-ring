//! Integration test for `RR-0043` (basic).
//! Ring buffer core refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0043_ring_buffer_core_refacto_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let first = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0043: Ring buffer core refactor mutator v18");
    let second = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0043: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0043: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0043: scanner should emit domain hints");
}
