//! Integration test for `RR-0532` (basic).
//! Extended: Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0532_ring_buffer_core_integra_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let first = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0532: Extended: Ring buffer core integrate validator v7");
    let second = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0532: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0532: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0532: stats visits every byte");
}
