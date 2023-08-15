//! Integration test for `RR-0053` (basic).
//! Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0053_ring_buffer_core_refacto_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let first = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0053: Ring buffer core refactor mutator v28");
    let second = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0053: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0053: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0053: scanner should emit domain hints");
}
