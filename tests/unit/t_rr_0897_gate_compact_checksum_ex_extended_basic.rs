//! Integration test for `RR-0897` (basic).
//! Extended: Gate compact checksum export harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0897_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x8c];
    let first = relayring::capabilities::rr_0897_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0897: Extended: Gate compact checksum export harden index v2");
    let second = relayring::capabilities::rr_0897_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0897: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0897: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0897: stats visits every byte");
}
