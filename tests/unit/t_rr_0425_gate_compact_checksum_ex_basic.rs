//! Integration test for `RR-0425` (basic).
//! Gate compact checksum export implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0425_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xae, 0xb0];
    let first = relayring::capabilities::rr_0425_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0425: Gate compact checksum export implement pipeline v30");
    let second = relayring::capabilities::rr_0425_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0425: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0425: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0425: window consumes the whole buffer");
}
