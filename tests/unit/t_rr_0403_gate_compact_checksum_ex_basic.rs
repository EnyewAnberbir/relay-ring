//! Integration test for `RR-0403` (basic).
//! Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0403_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let first = relayring::capabilities::rr_0403_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0403: Gate compact checksum export refactor mutator v8");
    let second = relayring::capabilities::rr_0403_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0403: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0403: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0403: stats visits every byte");
}
