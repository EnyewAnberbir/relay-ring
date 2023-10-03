//! Integration test for `RR-0396` (basic).
//! Gate compact checksum export extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0396_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let first = relayring::capabilities::rr_0396_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0396: Gate compact checksum export extend codec v1");
    let second = relayring::capabilities::rr_0396_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0396: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0396: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0396: stats visits every byte");
}
