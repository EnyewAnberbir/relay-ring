//! Integration test for `RR-0052` (basic).
//! Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0052_ring_buffer_core_integra_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let first = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("RR-0052: Ring buffer core integrate validator v27");
    let second = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("RR-0052: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0052: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0052: scanner should emit domain hints");
}
