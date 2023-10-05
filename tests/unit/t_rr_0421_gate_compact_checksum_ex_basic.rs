//! Integration test for `RR-0421` (basic).
//! Gate compact checksum export export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0421_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xac];
    let first = relayring::capabilities::rr_0421_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0421: Gate compact checksum export export adapter v26");
    let second = relayring::capabilities::rr_0421_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0421: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0421: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0421: stats visits every byte");
}
