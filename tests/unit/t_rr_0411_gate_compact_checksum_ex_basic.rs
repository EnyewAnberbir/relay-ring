//! Integration test for `RR-0411` (basic).
//! Gate compact checksum export export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0411_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let first = relayring::capabilities::rr_0411_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0411: Gate compact checksum export export adapter v16");
    let second = relayring::capabilities::rr_0411_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0411: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0411: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0411: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
