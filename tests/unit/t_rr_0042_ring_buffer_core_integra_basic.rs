//! Integration test for `RR-0042` (basic).
//! Ring buffer core integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0042_ring_buffer_core_integra_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let first = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(fixture).expect("RR-0042: Ring buffer core integrate validator v17");
    let second = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(fixture).expect("RR-0042: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0042: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0042: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
