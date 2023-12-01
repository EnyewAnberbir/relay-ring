//! Integration test for `RR-0857` (basic).
//! Extended: Gate surfaces append seek harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0857_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let first = relayring::capabilities::rr_0857_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0857: Extended: Gate surfaces append seek harden index v22");
    let second = relayring::capabilities::rr_0857_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0857: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0857: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0857: window consumes the whole buffer");
}
