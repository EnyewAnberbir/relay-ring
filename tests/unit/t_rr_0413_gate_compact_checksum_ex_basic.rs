//! Integration test for `RR-0413` (basic).
//! Gate compact checksum export refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0413_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa2, 0xa4];
    let first = relayring::capabilities::rr_0413_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0413: Gate compact checksum export refactor mutator v18");
    let second = relayring::capabilities::rr_0413_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0413: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0413: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0413: scanner should emit domain hints");
}
