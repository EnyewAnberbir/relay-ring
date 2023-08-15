//! Integration test for `RR-0054` (basic).
//! Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0054_ring_buffer_core_benchma_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let first = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0054: Ring buffer core benchmark reporter v29");
    let second = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0054: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0054: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0054: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
