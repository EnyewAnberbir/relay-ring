//! Integration test for `RR-0033` (basic).
//! Ring buffer core refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0033_ring_buffer_core_refacto_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let first = relayring::capabilities::rr_0033_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0033: Ring buffer core refactor mutator v8");
    let second = relayring::capabilities::rr_0033_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0033: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0033: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0033: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
