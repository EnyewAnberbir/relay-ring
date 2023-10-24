//! Integration test for `RR-0554` (basic).
//! Extended: Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0554_ring_buffer_core_benchma_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let first = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0554: Extended: Ring buffer core benchmark reporter v29");
    let second = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0554: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0554: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0554: stats visits every byte");
}
