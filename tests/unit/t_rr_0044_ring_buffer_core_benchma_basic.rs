//! Integration test for `RR-0044` (basic).
//! Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0044_ring_buffer_core_benchma_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let first = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0044: Ring buffer core benchmark reporter v19");
    let second = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0044: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0044: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0044: stats visits every byte");
}
