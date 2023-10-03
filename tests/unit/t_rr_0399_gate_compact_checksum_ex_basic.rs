//! Integration test for `RR-0399` (basic).
//! Gate compact checksum export optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0399_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96];
    let first = relayring::capabilities::rr_0399_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0399: Gate compact checksum export optimize registry v4");
    let second = relayring::capabilities::rr_0399_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0399: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0399: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0399: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
