//! Integration test for `RR-0397` (basic).
//! Gate compact checksum export harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0397_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x92, 0x94];
    let first = relayring::capabilities::rr_0397_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0397: Gate compact checksum export harden index v2");
    let second = relayring::capabilities::rr_0397_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0397: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0397: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0397: stats visits every byte");
}
