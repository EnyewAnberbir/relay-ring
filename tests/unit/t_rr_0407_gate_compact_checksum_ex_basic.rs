//! Integration test for `RR-0407` (basic).
//! Gate compact checksum export harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0407_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x9e];
    let first = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0407: Gate compact checksum export harden index v12");
    let second = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0407: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0407: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0407: window consumes the whole buffer");
}
