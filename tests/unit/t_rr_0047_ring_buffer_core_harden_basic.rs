//! Integration test for `RR-0047` (basic).
//! Ring buffer core harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0047_ring_buffer_core_harden_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let first = relayring::capabilities::rr_0047_ring_buffer_core_harden::evaluate(fixture).expect("RR-0047: Ring buffer core harden index v22");
    let second = relayring::capabilities::rr_0047_ring_buffer_core_harden::evaluate(fixture).expect("RR-0047: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0047: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0047: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
