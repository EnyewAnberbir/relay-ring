//! Integration test for `RR-0903` (basic).
//! Extended: Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0903_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let first = relayring::capabilities::rr_0903_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0903: Extended: Gate compact checksum export refactor mutator v8");
    let second = relayring::capabilities::rr_0903_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0903: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0903: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0903: stats visits every byte");
}
