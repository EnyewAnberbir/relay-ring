//! Integration test for `RR-0897` (empty).
//! Extended: Gate compact checksum export harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0897_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0897_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0897: empty input must fail for Extended: Gate compact checksum export harden index v2");
}
