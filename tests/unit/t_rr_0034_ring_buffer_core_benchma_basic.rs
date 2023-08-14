//! Integration test for `RR-0034` (basic).
//! Ring buffer core benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0034_ring_buffer_core_benchma_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let first = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0034: Ring buffer core benchmark reporter v9");
    let second = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0034: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0034: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0034: scanner should emit domain hints");
}
