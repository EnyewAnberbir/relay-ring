//! Integration test for `RR-0057` (basic).
//! Ring buffer core harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0057_ring_buffer_core_harden_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let first = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(fixture).expect("RR-0057: Ring buffer core harden index v32");
    let second = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(fixture).expect("RR-0057: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0057: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0057: stats visits every byte");
}
