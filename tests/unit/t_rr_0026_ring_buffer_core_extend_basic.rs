//! Integration test for `RR-0026` (basic).
//! Ring buffer core extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0026_ring_buffer_core_extend_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let first = relayring::capabilities::rr_0026_ring_buffer_core_extend::evaluate(fixture).expect("RR-0026: Ring buffer core extend codec v1");
    let second = relayring::capabilities::rr_0026_ring_buffer_core_extend::evaluate(fixture).expect("RR-0026: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0026: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0026: stats visits every byte");
}
