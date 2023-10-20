//! Integration test for `RR-0537` (basic).
//! Extended: Ring buffer core harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0537_ring_buffer_core_harden_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let first = relayring::capabilities::rr_0537_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0537: Extended: Ring buffer core harden index v12");
    let second = relayring::capabilities::rr_0537_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0537: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0537: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0537: stats visits every byte");
}
