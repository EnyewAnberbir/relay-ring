//! Integration test for `RR-0049` (basic).
//! Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0049_ring_buffer_core_optimiz_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let first = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0049: Ring buffer core optimize registry v24");
    let second = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0049: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0049: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0049: stats visits every byte");
}
