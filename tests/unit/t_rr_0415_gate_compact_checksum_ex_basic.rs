//! Integration test for `RR-0415` (basic).
//! Gate compact checksum export implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0415_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xa6];
    let first = relayring::capabilities::rr_0415_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0415: Gate compact checksum export implement pipeline v20");
    let second = relayring::capabilities::rr_0415_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0415: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0415: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0415: scanner should emit domain hints");
}
