//! Integration test for `RR-0544` (basic).
//! Extended: Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0544_ring_buffer_core_benchma_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let first = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0544: Extended: Ring buffer core benchmark reporter v19");
    let second = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0544: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0544: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0544: stats visits every byte");
}
