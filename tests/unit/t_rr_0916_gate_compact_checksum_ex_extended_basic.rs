//! Integration test for `RR-0916` (basic).
//! Extended: Gate compact checksum export extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0916_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x9f];
    let first = relayring::capabilities::rr_0916_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0916: Extended: Gate compact checksum export extend codec v21");
    let second = relayring::capabilities::rr_0916_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0916: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0916: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0916: window consumes the whole buffer");
}
