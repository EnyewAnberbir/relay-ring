//! Integration test for `RR-0410` (basic).
//! Gate compact checksum export validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0410_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0xa1];
    let first = relayring::capabilities::rr_0410_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0410: Gate compact checksum export validate resolver v15");
    let second = relayring::capabilities::rr_0410_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0410: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0410: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0410: window consumes the whole buffer");
}
