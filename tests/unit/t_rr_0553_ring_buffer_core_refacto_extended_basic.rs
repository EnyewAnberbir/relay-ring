//! Integration test for `RR-0553` (basic).
//! Extended: Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0553_ring_buffer_core_refacto_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let first = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0553: Extended: Ring buffer core refactor mutator v28");
    let second = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0553: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0553: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0553: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
