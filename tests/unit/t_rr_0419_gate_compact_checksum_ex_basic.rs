//! Integration test for `RR-0419` (basic).
//! Gate compact checksum export optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0419_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let first = relayring::capabilities::rr_0419_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0419: Gate compact checksum export optimize registry v24");
    let second = relayring::capabilities::rr_0419_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0419: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0419: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0419: window consumes the whole buffer");
}
