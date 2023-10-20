//! Integration test for `RR-0531` (basic).
//! Extended: Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0531_ring_buffer_core_export_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let first = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0531: Extended: Ring buffer core export adapter v6");
    let second = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0531: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0531: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0531: stats visits every byte");
}
