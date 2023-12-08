//! Integration test for `RR-0906` (basic).
//! Extended: Gate compact checksum export extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0906_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x93, 0x95];
    let first = relayring::capabilities::rr_0906_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0906: Extended: Gate compact checksum export extend codec v11");
    let second = relayring::capabilities::rr_0906_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0906: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0906: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0906: window consumes the whole buffer");
}
