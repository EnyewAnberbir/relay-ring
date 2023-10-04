//! Integration test for `RR-0401` (basic).
//! Gate compact checksum export export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0401_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let first = relayring::capabilities::rr_0401_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0401: Gate compact checksum export export adapter v6");
    let second = relayring::capabilities::rr_0401_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0401: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0401: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0401: scanner should emit domain hints");
}
