//! Integration test for `RR-0418` (basic).
//! Gate compact checksum export wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0418_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xa9];
    let first = relayring::capabilities::rr_0418_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0418: Gate compact checksum export wire planner v23");
    let second = relayring::capabilities::rr_0418_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0418: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0418: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0418: window consumes the whole buffer");
}
