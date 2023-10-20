//! Integration test for `RR-0534` (basic).
//! Extended: Ring buffer core benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0534_ring_buffer_core_benchma_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let first = relayring::capabilities::rr_0534_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0534: Extended: Ring buffer core benchmark reporter v9");
    let second = relayring::capabilities::rr_0534_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0534: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0534: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0534: scanner should emit domain hints");
}
