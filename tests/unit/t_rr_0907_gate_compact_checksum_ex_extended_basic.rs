//! Integration test for `RR-0907` (basic).
//! Extended: Gate compact checksum export harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0907_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96];
    let first = relayring::capabilities::rr_0907_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0907: Extended: Gate compact checksum export harden index v12");
    let second = relayring::capabilities::rr_0907_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0907: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0907: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0907: window consumes the whole buffer");
}
