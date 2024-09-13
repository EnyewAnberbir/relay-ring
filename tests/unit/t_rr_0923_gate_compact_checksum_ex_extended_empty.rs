//! Integration test for `RR-0923` (empty).
//! Extended: Gate compact checksum export refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0923_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0923_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0923: empty input must fail for Extended: Gate compact checksum export refactor mutator v28");
}
