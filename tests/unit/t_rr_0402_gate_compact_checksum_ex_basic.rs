//! Integration test for `RR-0402` (basic).
//! Gate compact checksum export integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0402_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let first = relayring::capabilities::rr_0402_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0402: Gate compact checksum export integrate validator v7");
    let second = relayring::capabilities::rr_0402_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0402: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0402: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0402: stats visits every byte");
}
