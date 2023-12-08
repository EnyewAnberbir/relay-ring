//! Integration test for `RR-0911` (basic).
//! Extended: Gate compact checksum export export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0911_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let first = relayring::capabilities::rr_0911_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0911: Extended: Gate compact checksum export export adapter v16");
    let second = relayring::capabilities::rr_0911_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0911: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0911: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0911: window consumes the whole buffer");
}
