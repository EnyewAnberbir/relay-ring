//! Integration test for `RR-0417` (empty).
//! Gate compact checksum export harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0417_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0417_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0417: empty input must fail for Gate compact checksum export harden index v22");
}
