//! Integration test for `RR-0923` (basic).
//! Extended: Gate compact checksum export refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0923_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xa6];
    let first = relayring::capabilities::rr_0923_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0923: Extended: Gate compact checksum export refactor mutator v28");
    let second = relayring::capabilities::rr_0923_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0923: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0923: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0923: stats visits every byte");
}
