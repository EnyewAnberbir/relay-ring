//! Integration test for `RR-0400` (basic).
//! Gate compact checksum export validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0400_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x97];
    let first = relayring::capabilities::rr_0400_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0400: Gate compact checksum export validate resolver v5");
    let second = relayring::capabilities::rr_0400_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0400: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0400: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0400: stats visits every byte");
}
